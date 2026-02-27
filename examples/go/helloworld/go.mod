module helloworld

go 1.21

require neutral_ipc_template v0.0.0

require (
	github.com/vmihailenco/msgpack/v5 v5.4.1 // indirect
	github.com/vmihailenco/tagparser/v2 v2.0.0 // indirect
)

replace neutral_ipc_template => ../neutral_ipc_template
