// ZIP32 writer using stored entries. Blob parts avoid one giant output buffer.
const encoder = new TextEncoder();
const crcTable = Uint32Array.from({length:256},(_,n)=>{let c=n;for(let i=0;i<8;i++)c=c&1?0xedb88320^(c>>>1):c>>>1;return c>>>0;});
const crc32 = bytes => {let c=0xffffffff;for(const b of bytes)c=crcTable[(c^b)&255]^(c>>>8);return (c^0xffffffff)>>>0;};
const u16=v=>[v&255,(v>>>8)&255], u32=v=>[v&255,(v>>>8)&255,(v>>>16)&255,(v>>>24)&255];
export class ZipStore {
  constructor(){this.parts=[];this.central=[];this.offset=0;this.count=0;}
  add(path,data){
    if(!/^[a-zA-Z0-9_./-]+$/.test(path)||path.includes("..")||path.startsWith("/"))throw Error("Invalid archive path");
    const name=encoder.encode(path), bytes=data instanceof Uint8Array?data:encoder.encode(data);
    if(name.length>65535||bytes.length>0xffffffff||this.offset+bytes.length+name.length+30>0xffffffff)throw Error("ZIP32 size limit exceeded");
    const crc=crc32(bytes), size=bytes.length, local=Uint8Array.from([...u32(0x04034b50),...u16(20),...u16(0),...u16(0),...u16(0),...u16(0),...u32(crc),...u32(size),...u32(size),...u16(name.length),...u16(0)]);
    this.parts.push(local,name,bytes);
    this.central.push(Uint8Array.from([...u32(0x02014b50),...u16(20),...u16(20),...u16(0),...u16(0),...u16(0),...u16(0),...u32(crc),...u32(size),...u32(size),...u16(name.length),...u16(0),...u16(0),...u16(0),...u16(0),...u32(0),...u32(this.offset)]),name);
    this.offset+=local.length+name.length+size;this.count++;
  }
  blob(){if(this.count>65535)throw Error("ZIP32 entry limit exceeded");const centralSize=this.central.reduce((sum,p)=>sum+p.length,0);const end=Uint8Array.from([...u32(0x06054b50),...u16(0),...u16(0),...u16(this.count),...u16(this.count),...u32(centralSize),...u32(this.offset),...u16(0)]);return new Blob([...this.parts,...this.central,end],{type:"application/zip"});}
}
