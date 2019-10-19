const http = require('http');
const querystring = require('querystring');
const SOCKETFILE = process.env.SOCKETFILE || __dirname + '/node.sock';


const postData = querystring.stringify({
  'msg': 'Hello World!'
});
const options = {
  socketPath: SOCKETFILE,
  path: 'ping',
  auth: 'myauth',
  method: 'post',
  headers: {
    'Content-Type': 'application/x-www-form-urlencoded',
    'Content-Length': Buffer.byteLength(postData)
  }
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
clientRequest.write(postData);
clientRequest.end();