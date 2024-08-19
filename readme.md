1.修改.env的rpc为你的rpc，WALLET_PATH 修改为你的json文件，比如 1.json
2.默认使用jito，手续费最低的
3.服务端执行命令（优先费为0即可） .\ore-hq-server.exe --priority-fee 0
3.客户端执行命令（2.json随便用一个空钱包就好了，所有客户端都用同一个空钱包就行） .\ore-hq-client.exe --url ws://127.0.0.1:3000 --keypair 2.json --threads 20
5.主程序需要.env 1.json ore-hq-server.exe
6.客户端 ore-hq-client.exe 2.json

