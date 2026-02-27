package neutral_ipc_template

import (
	"bytes"
	"encoding/binary"
	"encoding/json"
	"fmt"
	"net"
	"time"

	msgpack "github.com/vmihailenco/msgpack/v5"
)

// NeutralIpcRecord constants and methods.
const (
	Reserved          = 0
	HeaderLen         = 12
	CtrlParseTemplate = 10
	CtrlStatusOk      = 0
	CtrlStatusKo      = 1
	ContentJSON       = 10
	ContentMsgpack    = 50
	ContentPath       = 20
	ContentText       = 30
	ContentBin        = 40
	Empty             = ""
)

type NeutralIpcRecord struct {
	Reserved byte
	Control  byte
	Format1  byte
	Length1  uint32
	Format2  byte
	Length2  uint32
}

func (r *NeutralIpcRecord) DecodeHeader(recordHeader []byte) error {
	if len(recordHeader) != HeaderLen {
		return fmt.Errorf("invalid header length")
	}
	r.Reserved = recordHeader[0]
	r.Control = recordHeader[1]
	r.Format1 = recordHeader[2]
	r.Length1 = binary.BigEndian.Uint32(recordHeader[3:7])
	r.Format2 = recordHeader[7]
	r.Length2 = binary.BigEndian.Uint32(recordHeader[8:12])
	return nil
}

func (r *NeutralIpcRecord) EncodeHeader() []byte {
	header := make([]byte, HeaderLen)
	header[0] = Reserved
	header[1] = r.Control
	header[2] = r.Format1
	binary.BigEndian.PutUint32(header[3:7], r.Length1)
	header[7] = r.Format2
	binary.BigEndian.PutUint32(header[8:12], r.Length2)
	return header
}

func (r *NeutralIpcRecord) EncodeRecord(control byte, format1 byte, content1 interface{}, format2 byte, content2 string) []byte {
	r.Control = control
	r.Format1 = format1

	var content1Bytes []byte
	switch c := content1.(type) {
	case string:
		content1Bytes = []byte(c)
	case []byte:
		content1Bytes = c
	default:
		content1Bytes = []byte(c.(string))
	}

	r.Length1 = uint32(len(content1Bytes))
	r.Format2 = format2
	r.Length2 = uint32(len(content2))
	header := r.EncodeHeader()
	return append(header, append(content1Bytes, []byte(content2)...)...)
}

func (r *NeutralIpcRecord) DecodeRecord(header []byte, content1, content2 string) map[string]interface{} {
	if err := r.DecodeHeader(header); err != nil {
		return nil
	}
	return map[string]interface{}{
		"reserved":  r.Reserved,
		"control":   r.Control,
		"format-1":  r.Format1,
		"content-1": content1,
		"format-2":  r.Format2,
		"content-2": content2,
	}
}

// NeutralIpcClient handles IPC communication.
type NeutralIpcClient struct {
	control  byte
	format1  byte
	content1 string
	format2  byte
	content2 string
	result   map[string]interface{}
}

func NewNeutralIpcClient(control byte, format1 byte, content1 string, format2 byte, content2 string) *NeutralIpcClient {
	return &NeutralIpcClient{
		control:  control,
		format1:  format1,
		content1: content1,
		format2:  format2,
		content2: content2,
	}
}

func (c *NeutralIpcClient) Start() (map[string]interface{}, error) {
	conn, err := net.DialTimeout("tcp", fmt.Sprintf("%s:%d", GetHost(), GetPort()), time.Duration(GetTimeout())*time.Second)
	if err != nil {
		return nil, err
	}
	defer conn.Close()

	record := &NeutralIpcRecord{}
	request := record.EncodeRecord(c.control, c.format1, c.content1, c.format2, c.content2)
	if _, err := conn.Write(request); err != nil {
		return nil, err
	}

	responseHeader := make([]byte, HeaderLen)
	if _, err := conn.Read(responseHeader); err != nil {
		return nil, err
	}

	respRecord := &NeutralIpcRecord{}
	if err := respRecord.DecodeHeader(responseHeader); err != nil {
		return nil, err
	}

	content1, err := c.readContent(conn, int(respRecord.Length1))
	if err != nil {
		return nil, err
	}
	content2, err := c.readContent(conn, int(respRecord.Length2))
	if err != nil {
		return nil, err
	}

	c.result = respRecord.DecodeRecord(responseHeader, content1, content2)
	return c.result, nil
}

func (c *NeutralIpcClient) readContent(conn net.Conn, length int) (string, error) {
	if length == 0 {
		return "", nil
	}
	chunks := make([][]byte, 0)
	remaining := length

	for remaining > 0 {
		n := min(GetBufferSize(), remaining)
		chunk := make([]byte, n)
		read := 0
		for read < n {
			chunkRead, err := conn.Read(chunk[read:])
			if err != nil {
				return "", err
			}
			read += chunkRead
		}
		chunks = append(chunks, chunk)
		remaining -= n
	}

	var buffer bytes.Buffer
	for _, chunk := range chunks {
		buffer.Write(chunk)
	}
	return buffer.String(), nil
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

// NeutralIpcTemplate is the main IPC template class.
type NeutralIpcTemplate struct {
	template   string
	tplType    byte
	schemaType byte
	schema     interface{}
	result     map[string]interface{}
}

func NewNeutralIpcTemplate(template string, schema interface{}) *NeutralIpcTemplate {
	return NewNeutralIpcTemplateWithSchemaType(template, schema, ContentJSON)
}

func NewNeutralIpcTemplateWithSchemaType(template string, schema interface{}, schemaType byte) *NeutralIpcTemplate {
	var schemaData interface{}

	if schemaType == ContentMsgpack {
		switch s := schema.(type) {
		case string:
			var parsed interface{}
			if err := json.Unmarshal([]byte(s), &parsed); err == nil {
				data, _ := msgpack.Marshal(parsed)
				schemaData = data
			} else {
				schemaData = []byte{}
			}
		case map[string]interface{}:
			data, _ := msgpack.Marshal(s)
			schemaData = data
		default:
			schemaData = []byte{}
		}
	} else {
		if s, ok := schema.(string); ok {
			schemaData = s
		} else if sch, ok := schema.(map[string]interface{}); ok {
			data, _ := json.Marshal(sch)
			schemaData = string(data)
		}
	}

	return &NeutralIpcTemplate{
		template:   template,
		tplType:    ContentPath,
		schemaType: schemaType,
		schema:     schemaData,
	}
}

func (t *NeutralIpcTemplate) Render() string {
	var schemaStr string
	switch s := t.schema.(type) {
	case string:
		schemaStr = s
	case []byte:
		schemaStr = string(s)
	default:
		schemaStr = ""
	}

	client := NewNeutralIpcClient(CtrlParseTemplate, t.schemaType, schemaStr, t.tplType, t.template)
	result, err := client.Start()
	if err != nil {
		t.result = map[string]interface{}{"error": err.Error()}
		return ""
	}
	t.result = result

	status, ok := result["control"].(byte)
	if !ok {
		t.result["error"] = "invalid control type in result"
		return ""
	}
	content1, ok := result["content-1"].(string)
	if !ok {
		t.result["error"] = "invalid content-1 type in result"
		return ""
	}
	content2, ok := result["content-2"].(string)
	if !ok {
		t.result["error"] = "invalid content-2 type in result"
		return ""
	}

	var res map[string]interface{}
	if err := json.Unmarshal([]byte(content1), &res); err != nil {
		t.result["result"] = content1
	} else {
		t.result["result"] = res
	}
	t.result["status"] = status
	t.result["content"] = content2

	return t.result["content"].(string)
}

func (t *NeutralIpcTemplate) SetPath(path string) {
	t.tplType = ContentPath
	t.template = path
}

func (t *NeutralIpcTemplate) SetSource(source string) {
	t.tplType = ContentText
	t.template = source
}

func (t *NeutralIpcTemplate) MergeSchema(schema interface{}) {
	if t.schemaType == ContentMsgpack {
		return // Cannot merge msgpack schemas
	}

	schemaStr := ""
	if s, ok := schema.(string); ok {
		schemaStr = s
	} else if sch, ok := schema.(map[string]interface{}); ok {
		data, _ := json.Marshal(sch)
		schemaStr = string(data)
	}

	var current map[string]interface{}
	switch s := t.schema.(type) {
	case string:
		if err := json.Unmarshal([]byte(s), &current); err != nil {
			t.result = map[string]interface{}{"error": "failed to unmarshal current schema: " + err.Error()}
			return
		}
	default:
		return
	}
	var newSch map[string]interface{}
	if err := json.Unmarshal([]byte(schemaStr), &newSch); err != nil {
		t.result = map[string]interface{}{"error": "failed to unmarshal new schema: " + err.Error()}
		return
	}

	merged := deepMerge(current, newSch)
	mergedData, err := json.Marshal(merged)
	if err != nil {
		t.result = map[string]interface{}{"error": "failed to marshal merged schema: " + err.Error()}
		return
	}
	t.schema = string(mergedData)
}

func (t *NeutralIpcTemplate) HasError() bool {
	if t.result == nil {
		return true
	}
	status, ok := t.result["status"].(byte)
	if !ok || status != CtrlStatusOk {
		return true
	}
	res, ok := t.result["result"].(map[string]interface{})
	if ok {
		if hasError, ok := res["has_error"].(bool); ok && hasError {
			return true
		}
	}
	return false
}

func (t *NeutralIpcTemplate) GetStatusCode() interface{} {
	if t.result == nil {
		return nil
	}
	res, ok := t.result["result"].(map[string]interface{})
	if !ok {
		return nil
	}
	return res["status_code"]
}

func (t *NeutralIpcTemplate) GetStatusText() interface{} {
	if t.result == nil {
		return nil
	}
	res, ok := t.result["result"].(map[string]interface{})
	if !ok {
		return nil
	}
	return res["status_text"]
}

func (t *NeutralIpcTemplate) GetStatusParam() interface{} {
	if t.result == nil {
		return nil
	}
	res, ok := t.result["result"].(map[string]interface{})
	if !ok {
		return nil
	}
	return res["status_param"]
}

func (t *NeutralIpcTemplate) GetResult() map[string]interface{} {
	if t.result == nil {
		return nil
	}
	res, ok := t.result["result"].(map[string]interface{})
	if !ok {
		return nil
	}
	return res
}

// deepMerge merges two maps deeply.
func deepMerge(dict1, dict2 map[string]interface{}) map[string]interface{} {
	merged := make(map[string]interface{})
	for k, v := range dict1 {
		merged[k] = v
	}

	for k, v := range dict2 {
		if val1, ok := merged[k]; ok {
			if m1, ok1 := val1.(map[string]interface{}); ok1 {
				if m2, ok2 := v.(map[string]interface{}); ok2 {
					merged[k] = deepMerge(m1, m2)
					continue
				}
			}
		}
		merged[k] = v
	}
	return merged
}
