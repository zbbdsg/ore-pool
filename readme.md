1.修改.env的rpc为你的rpc，WALLET_PATH 修改为你的json文件，比如 1.json，其他参数不用管
2.默认使用jito，手续费最低的（只能用jito，哈哈哈）
3.服务端执行命令（优先费为0即可） .\ore-hq-server.exe --priority-fee 0
3.客户端执行命令（2.json随便用一个空钱包就好了，所有客户端都用同一个空钱包就行） .\ore-hq-client.exe --url ws://127.0.0.1:3000 --keypair 2.json --threads 20
5.主程序需要.env 1.json ore-hq-server.exe
6.客户端 ore-hq-client.exe 2.json

备注：
我只是加了jito，优化了一下上链，把ore的上链代码搬过来，花了点时间调试，这个版本我也是自己测试过，还算稳定，不会存在不上链导致程序错乱
(因为之前测试了一下原版本，上链老是不成功，会导致程序错乱，只能自己改代码了)

遇到的坑：
就是客户端时间跟主程序的服务器的时间不一致，最好都是同一个机房，这样就能保证时间一致即可，如果不一致，就把本地时间更新一下就好了。

如何想要不看代码，直接测试也可以，只用拷贝fast_mint下的文件，只要填写1.json和2.json的私钥就可以了。

该代码来源于https://github.com/Kriptikz，如有侵权，请联系我，立马删除
