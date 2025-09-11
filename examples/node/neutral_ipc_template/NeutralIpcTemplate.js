/**
 * Neutral Node.js IPC client for Neutral TS.
 * https://github.com/FranBarInstance/neutral-ipc
 */

const net = require('net');
const { HOST, PORT, TIMEOUT, BUFFER_SIZE } = require('./NeutralIpcConfig');

class NeutralIpcRecord {

    // ============================================
    // Neutral IPC record version 0 (draft version)
    // ============================================
    //
    // HEADER:
    //
    // \x00              # reserved
    // \x00              # control (action/status) (10 = parse template)
    // \x00              # content-format 1 (10 = JSON, 20 = file path, 30 = plaintext, 40 = binary)
    // \x00\x00\x00\x00  # content-length 1 big endian byte order
    // \x00              # content-format 2 (10 = JSON, 20 = file path, 30 = plaintext, 40 = binary)
    // \x00\x00\x00\x00  # content-length 2 big endian byte order (can be zero)
    //
    // All text utf8


    static RESERVED = 0;
    static HEADER_LEN = 12;
    static CTRL_PARSE_TEMPLATE = 10;
    static CTRL_STATUS_OK = 0;
    static CTRL_STATUS_KO = 1;
    static CONTENT_JSON = 10;
    static CONTENT_PATH = 20;
    static CONTENT_TEXT = 30;
    static CONTENT_BIN = 40;

    static decodeHeader(recordHeader) {
        const reserved = recordHeader[0];
        const control = recordHeader[1];
        const format1 = recordHeader[2];
        const length1 = recordHeader.readUInt32BE(3);
        const format2 = recordHeader[7];
        const length2 = recordHeader.readUInt32BE(8);
        return {
            'reserved': reserved,
            'control': control,
            'format-1': format1,
            'length-1': length1,
            'format-2': format2,
            'length-2': length2,
        };
    }

    static encodeHeader(control, format1, length1, format2, length2) {
        const header = Buffer.alloc(this.HEADER_LEN);
        header[0] = this.RESERVED;
        header[1] = control;
        header[2] = format1;
        header.writeUInt32BE(length1, 3);
        header[7] = format2;
        header.writeUInt32BE(length2, 8);
        return header;
    }

    static encodeRecord(control, format1, content1, format2, content2) {
        const length1 = Buffer.byteLength(content1, 'utf8');
        const length2 = Buffer.byteLength(content2, 'utf8');
        const header = this.encodeHeader(control, format1, length1, format2, length2);
        const content1Buf = Buffer.from(content1, 'utf8');
        const content2Buf = Buffer.from(content2, 'utf8');
        return Buffer.concat([header, content1Buf, content2Buf]);
    }

    static decodeRecord(header, content1, content2) {
        const decodedHeader = this.decodeHeader(header);
        return {
            "reserved": this.RESERVED,
            "control": decodedHeader['control'],
            'format-1': decodedHeader['format-1'],
            'content-1': content1,
            'format-2': decodedHeader['format-2'],
            'content-2': content2,
        };
    }
}

class NeutralIpcClient {
    constructor(control, format1, content1, format2, content2) {
        this.control = control;
        this.format1 = format1;
        this.content1 = content1;
        this.format2 = format2;
        this.content2 = content2;
        this.result = {};
    }

    async start() {
        return new Promise((resolve, reject) => {
            const socket = net.createConnection({ host: HOST, port: PORT, timeout: TIMEOUT }, () => {
                const request = NeutralIpcRecord.encodeRecord(
                    this.control, this.format1, this.content1, this.format2, this.content2
                );
                socket.write(request);
            });

            socket.setTimeout(TIMEOUT);
            socket.on('timeout', () => {
                socket.destroy();
                reject(new Error('Socket timeout'));
            });
            socket.on('error', reject);

            let responseHeader;
            let totalReceived = 0;
            let content1 = '';
            let content2 = '';
            let readingHeader = true;
            let headerReceived = false;
            let length1 = 0;
            let length2 = 0;
            let content1Received = 0;
            let content2Received = 0;

            socket.on('data', (data) => {
                if (readingHeader) {
                    if (!responseHeader) {
                        responseHeader = Buffer.alloc(NeutralIpcRecord.HEADER_LEN);
                    }
                    const toCopy = Math.min(data.length, NeutralIpcRecord.HEADER_LEN - totalReceived);
                    data.copy(responseHeader, totalReceived, 0, toCopy);
                    totalReceived += toCopy;
                    data = data.slice(toCopy);

                    if (totalReceived === NeutralIpcRecord.HEADER_LEN) {
                        const response = NeutralIpcRecord.decodeHeader(responseHeader);
                        length1 = response['length-1'];
                        length2 = response['length-2'];
                        readingHeader = false;
                        headerReceived = true;
                    }
                }

                if (headerReceived) {
                    if (content1Received < length1) {
                        const toCopy = Math.min(data.length, length1 - content1Received);
                        const buf = Buffer.from(data.slice(0, toCopy));
                        content1 += buf.toString('utf8');
                        content1Received += toCopy;
                        data = data.slice(toCopy);
                    }

                    if (content1Received === length1 && content2Received < length2) {
                        const toCopy = Math.min(data.length, length2 - content2Received);
                        const buf = Buffer.from(data.slice(0, toCopy));
                        content2 += buf.toString('utf8');
                        content2Received += toCopy;
                        data = data.slice(toCopy);
                    }

                    if (content1Received === length1 && content2Received === length2) {
                        socket.end();
                        this.result = NeutralIpcRecord.decodeRecord(responseHeader, content1, content2);
                        resolve(this.result);
                    }
                }
            });

            socket.on('end', () => {
                if (!headerReceived || content1Received < length1 || content2Received < length2) {
                    reject(new Error('Incomplete response'));
                }
            });
        });
    }
}

class NeutralIpcTemplate {
    constructor(template, schema, tplType = NeutralIpcRecord.CONTENT_PATH) {
        this.template = template;
        this.tplType = tplType;
        this.schema = typeof schema === 'object' ? JSON.stringify(schema) : schema;
        this.result = {};
    }

    async render() {
        const record = new NeutralIpcClient(
            NeutralIpcRecord.CTRL_PARSE_TEMPLATE,
            NeutralIpcRecord.CONTENT_JSON,
            this.schema,
            this.tplType,
            this.template
        );
        const result = await record.start();
        this.result = {
            'status': result['control'],
            'result': JSON.parse(result['content-1']),
            'content': result['content-2'],
        };
        return this.result['content'];
    }

    setPath(path) {
        this.tplType = NeutralIpcRecord.CONTENT_PATH;
        this.template = path;
    }

    setSource(source) {
        this.tplType = NeutralIpcRecord.CONTENT_TEXT;
        this.template = source;
    }

    mergeSchema(schema) {
        const currentSchema = JSON.parse(this.schema);
        const newSchema = typeof schema === 'string' ? JSON.parse(schema) : schema;
        this.schema = JSON.stringify(deepMerge(currentSchema, newSchema));
    }

    hasError() {
        return this.result.status !== 0 || (this.result.result && this.result.result.has_error);
    }

    getStatusCode() {
        return this.result.result ? this.result.result.status_code : null;
    }

    getStatusText() {
        return this.result.result ? this.result.result.status_text : null;
    }

    getStatusParam() {
        return this.result.result ? this.result.result.status_param : null;
    }

    getResult() {
        return this.result.result || null;
    }
}

function deepMerge(dict1, dict2) {
    const merged = { ...dict1 };
    for (const [key, value] of Object.entries(dict2)) {
        if (merged[key] && typeof merged[key] === 'object' && typeof value === 'object' && !Array.isArray(merged[key]) && !Array.isArray(value)) {
            merged[key] = deepMerge(merged[key], value);
        } else {
            merged[key] = value;
        }
    }
    return merged;
}

module.exports = NeutralIpcTemplate;
