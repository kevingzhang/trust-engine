/*
**
**  Example of Interprocess communication in Node.js through a UNIX domain socket
**
**  Usage:
**   server>  MODE=server node ipc.example.js
**   client>  MODE=client node ipc.example.js
**
*/
const net = require('net');
const fs = require('fs');
const connections = {};


// prevent duplicate exit messages
var SHUTDOWN = false;

// Our socket
const SOCKETFILE = process.env.SOCKETFILE || '/socket/node.sock';

mode = 'server';
console.info('Loading interprocess communications test');
console.info(' Server \n  Socket: %s \n  Process: %s',mode,SOCKETFILE,process.pid);

function createServer(socket){
    console.log('Creating server.');
    var server = net.createServer(function(stream) {
        console.log('Connection acknowledged.');

        // Store all connections so we can terminate them if the server closes.
        // An object is better than an array for these.
        var self = Date.now();
        connections[self] = (stream);
        stream.on('end', function() {
            console.log('Client disconnected.');
            delete connections[self];
        });

        // Messages are buffers. use toString
        stream.on('data', function(msg) {
            msg = msg.toString();
            if(msg === '__snootbooped'){
                console.log("Client's snoot confirmed booped.");
                return;
            }

            console.log('Client:', msg);

            if(msg === 'foo'){
                stream.write('bar');
            }

            if(msg === 'baz'){
                stream.write('qux');
            }

            if(msg === 'here come dat boi'){
                stream.write('Kill yourself.');
            }
            if(msg.startsWith("Ping")){
                stream.write("Pong response to " + msg);
            }
        });
    })
    .listen(socket)
    .on('connection', function(socket){
        console.log('Client connected.');
        console.log('Sending boop.');
        socket.write('__boop');
        //console.log(Object.keys(socket));
    })
    ;
    return server;
}


// check for failed cleanup
console.log('Checking for leftover socket.');
fs.stat(SOCKETFILE, function (err, stats) {
    if (err) {
        // start server
        console.log('No leftover socket found.');
        const server = createServer(SOCKETFILE); return;
    }
    else{
        // remove file then start server
        console.log('Removing leftover socket.')
        fs.unlink(SOCKETFILE, function(err){
            if(err){
                console.log("fs.unlink(SOCKETFILE) error. This should never happen.");
                console.error(err); 
                process.exit(0);
            }
            const server = createServer(SOCKETFILE); return;
        });  
    }
    
});

// close all connections when the user does CTRL-C
function cleanup(){
    if(!SHUTDOWN){ SHUTDOWN = true;
        console.log('\n',"Terminating.",'\n');
        if(Object.keys(connections).length){
            let clients = Object.keys(connections);
            while(clients.length){
                let client = clients.pop();
                connections[client].write('__disconnect');
                connections[client].end(); 
            }
        }
        server.close();
        process.exit(0);
    }
}
process.on('SIGINT', cleanup);

