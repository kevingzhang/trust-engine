const http = require('http');
const SOCKETFILE = process.env.SOCKETFILE || __dirname + '/node.sock';
const querystring = require('querystring');


const options = {
  socketPath: SOCKETFILE,
  path: '/get_rand_secret',
  auth: 'myauth'
};

const callback = res => {
  console.log(`STATUS: ${res.statusCode}`);
  res.setEncoding('utf8');
  res.on('data', data => {
    
    const bf = new Buffer.from(data);
    console.log("response buffer:", bf.toString('utf8'));
    const keys = JSON.parse(data);
    const publicKey = keys.public_key;
    const secretKey = keys.secret_key;
    getVrfProof(publicKey, secretKey);
    
  });
  res.on('error', data => console.error(data));

};
console.log("Before request", options)
const clientRequest = http.request(options, callback);
console.log("After request");
clientRequest.end();


const postVrfProofUrl = (publicKey, secretKey)=>{
  console.log(publicKey, secretKey);
  const postData = querystring.stringify({
    public_key: publicKey,
    secret_key: secretKey
  });
  const option = {
    socketPath: SOCKETFILE,
    path: "/get_vrf_proof",
    method: 'post',
    headers: {
      'Content-Type': 'application/x-www-form-urlencoded',
      'Content-Length': Buffer.byteLength(postData)
    }
  };
  const callback = res=>{
    console.log(`STATUS: ${res.statusCode}`);
    res.on('data', data=>console.log(data));
    res.on('err', data=>consoleerr(data));
  };
  http.request(option, callback).end();
}

const getVrfProof = (publicKey, secretKey)=>{
  //console.log(publicKey, secretKey);
  const msg = "sample";
  const path = '/get_vrf_proof?' + "p=" + publicKey + "&s=" + secretKey + "&m=" + msg; 
  const option = {
    socketPath: SOCKETFILE,
    path
  };
  const callback = res=>{
    console.log(`STATUS: ${res.statusCode}`);
    res.on('data', data=>{
      const res = new Buffer.from(data).toString('utf8');
      console.log("response", res);
      const resJson = JSON.parse(res);
      getVrfVerified(resJson.pi, publicKey, msg);
    });
    res.on('err', data=>console.error(data));
  };
  http.request(option, callback).end();
}

const getVrfVerified = (pi, publicKey, message) =>{
  const path = '/get_vrf_verified?' + "p=" + publicKey + "&pi=" + pi + "&m=" + message; 
  const option = {
    socketPath: SOCKETFILE,
    path
  };
  const callback = res=>{
    console.log(`STATUS: ${res.statusCode}`);
    res.on('data', data=>{
      const bf = new Buffer.from(data);
      console.log("response buffer:", bf.toString('utf8'));
    });
    res.on('err', data=>console.error(data));
  };
  http.request(option, callback).end();
}