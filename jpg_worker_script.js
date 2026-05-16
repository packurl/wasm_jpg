importScripts('./jpg_for_importScripts.js');
(async()=>{
  const fn=await jpg;
  onmessage=async({data:{bytes,width,height,quality}})=>{
    const res=fn(bytes,width,height,quality);
    postMessage(res,[res.buffer]);
  }
  postMessage('ready');
})();