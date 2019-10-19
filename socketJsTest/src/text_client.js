/*
**
**  Example of Interprocess communication in Node.js through a UNIX domain socket
**
**  Usage:
**   server>  MODE=server node ipc.example.js
**   client>  MODE=client node ipc.example.js
**
*/

var net = require('net'),
    fs = require('fs'),
    connections = {},
    server, client, mode
    ;

// prevent duplicate exit messages
var SHUTDOWN = false;

// Our socket
const SOCKETFILE = process.env.SOCKETFILE || __dirname + '/node.sock';
mode = "client";
// For simplicity of demonstration, both ends in this one file


console.info('Loading interprocess communications test');
console.info('  Mode: %s \n  Socket: %s \n  Process: %s',mode,SOCKETFILE,process.pid);

const tryConnect = (pid)=>{
    // Connect to server.
    console.log("Connecting to server.");
    
    const client = net.createConnection(SOCKETFILE)
    .on('connect', ()=>{
        console.log("Connected.");
        interactivesAfterConnection(client);
        loopPingMessage(client, pid);
    })
    // Messages are buffers. use toString
    .on('data', function(data) {
        data = data.toString();

        if(data === '__boop'){
            console.info('Server sent boop. Confirming our snoot is booped.');
            client.write('__snootbooped');
            return;
        }
        if(data === '__disconnect'){
            console.log('Server disconnected.')
            return cleanup(client);
        }

        // Generic message handler
        console.info('Server:', data)
    })
    .on('error', function(data) {
        console.error('Server not active. Will try again after 15 seconds');
        setTimeout(tryConnect, 15000);
    })
}

const interactivesAfterConnection = (client)=>{
         // Handle input from stdin.
    var inputbuffer = "";
    process.stdin.on("data", function (data) {
        inputbuffer += data;
        if (inputbuffer.indexOf("\n") !== -1) {
            var line = inputbuffer.substring(0, inputbuffer.indexOf("\n"));
            inputbuffer = inputbuffer.substring(inputbuffer.indexOf("\n") + 1);
            // Let the client escape
            if(line === 'exit'){ return cleanup(client); }
            if(line === 'quit'){ return cleanup(client); }
            client.write(line);
        }
    });


    process.on('SIGINT', cleanup);
};
function cleanup(client){
    if(!SHUTDOWN){ SHUTDOWN = true;
        console.log('\n',"Terminating.",'\n');
        client.end();
        process.exit(0);
    }
}
const loopPingMessage = (client, pid)=>{
    setTimeout(()=>{
        client.write("Ping from " + pid);
        loopPingMessage(client, pid);
    }, 10000);
}

tryConnect(process.pid);