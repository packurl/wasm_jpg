// jpeg encoding implementation from the rust heic crate (https://github.com/imazen/zenjepg)
// under the AGPL3 license (https://github.com/imazen/zenjpeg/blob/main/LICENSE-AGPL3)
const url=new URL('jpg.wasm',import.meta.url);
await (await fetch(url)).arrayBuffer();
const worker=await new Promise(r=>{
  // For browsers that don't support type: module on workers (firefox < 114, safari < 15)
  // const worker=new Worker(new URL('./jpg_worker_script.js',import.meta.url));
  const worker=new Worker(new URL('./jpg_worker_script.mjs',import.meta.url),{type:'module'});
  worker.onmessage=msg=>{
    if(msg.data==='ready'){
      worker.onmessage=null;
      r(worker);
    }
  };
});
/**
 * Encodes the supplied ImageData rgba array.
 * @param {Uint8Array} bytes
 * @param {number} width
 * @param {number} height
 * @param {number} [quality=80] (1 to 100)
 * @param {boolean} [transfer=false]
 * @return {Promise<Uint8Array>}
 */
const jpg=(bytes,width,height,quality=80,transfer=false)=>new Promise(r=>{
  worker.onmessage=msg=>{
    worker.onmessage=null;
    r(msg.data);
  }
  worker.postMessage({bytes,width,height,quality},transfer?[bytes.buffer]:undefined);
});

export {jpg};
export default jpg;
