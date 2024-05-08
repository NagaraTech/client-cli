Usage:

1. git clone
2. cargo install --path client-cli

Commands:

Zclient 
    - keystore
        - new <name> create a new keypair with name, if first create, it also will set the new keypair as default config
        - import <path> import a keypair json file to default keystore path
        - list show the keypairs
    - config
        - set-keypair <keypair name> set as current cli keypair
    - messager
        - set-ws-connection <URL> set the websocket connection
        - send-msg <FROM> <TO> <MSG>  send the message
        - receive-message a loop to receive message
        

