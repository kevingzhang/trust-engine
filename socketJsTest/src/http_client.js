const http = require('http');
const SOCKETFILE = process.env.SOCKETFILE || __dirname + '/node.sock';

const options = {
  socketPath: SOCKETFILE,
  path: "",//'/ping',
};

const callback = res => {
  console.log(`STATUS: ${res.statusCode}`);
  res.setEncoding('utf8');
  res.on('data', data => console.log(data));
  res.on('error', data => console.error(data));

};
console.log("Before request")
const clientRequest = http.request(options, callback);
console.log("After request");
clientRequest.end();