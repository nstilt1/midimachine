// Local drum synthesis: saturated low-end, snare body + noise, metallic hats.
export function builtInDrum(ctx, instrument) {
  const rate=ctx.sampleRate, length=instrument===2?.55:instrument===0?.5:.25;
  const buffer=ctx.createBuffer(1,Math.ceil(rate*length),rate), out=buffer.getChannelData(0);
  let phase=0, previous=0, seed=1337+instrument;
  for(let i=0;i<out.length;i++) {
    const t=i/rate;
    seed=(Math.imul(seed,1664525)+1013904223)|0;
    const noise=seed/2147483648, high=noise-previous; previous=noise;
    if(instrument===0) {phase+=2*Math.PI*(46+115*Math.exp(-t*45))/rate;out[i]=Math.tanh(2*Math.sin(phase))*Math.exp(-t*10)*.8+high*Math.exp(-t*220)*.12;}
    else if(instrument>=1&&instrument<=3) {const decay=instrument===2?9:instrument===3?40:65;out[i]=(high*.35+[4021,5393,6749].reduce((v,f)=>v+Math.sign(Math.sin(2*Math.PI*f*t))*.09,0))*Math.exp(-t*decay);}
    else if(instrument===4||instrument===5||instrument===6) {const body=Math.sin(2*Math.PI*(instrument===5?225:180)*t)*Math.exp(-t*28);const clap=instrument===6?(1+.5*Math.cos(t*1100))*Math.exp(-t*18):Math.exp(-t*21);out[i]=body*.45+high*clap*.38;}
    else {phase+=2*Math.PI*(90+(instrument-7)*45+90*Math.exp(-t*30))/rate;out[i]=Math.sin(phase)*Math.exp(-t*17)*.8;}
    out[i]*=Math.min(1,t*1500);
  }
  return buffer;
}
export class DrumAudio {
  constructor(){this.ctx=new AudioContext();this.samples=new Map();this.kit=new Map();this.sources=new Set();this.timer=null;}
  async load(index,file){const buffer=await this.ctx.decodeAudioData(await file.arrayBuffer());this.samples.set(index,buffer);}
  stop(){clearInterval(this.timer);this.timer=null;for(const s of this.sources){try{s.stop();}catch{}}this.sources.clear();this.output?.disconnect();}
  async play(pattern,loop=true){
    this.stop();await this.ctx.resume();
    const {config,hits}=pattern, duration=config.bars*240/config.bpm;
    const output=this.ctx.createGain();this.output=output;output.gain.value=.65;output.connect(this.ctx.destination);
    let next=this.ctx.currentTime+.06;
    const schedule=()=>{while(next<this.ctx.currentTime+.15){for(const h of hits){const s=this.ctx.createBufferSource(),g=this.ctx.createGain();if(!this.kit.has(h.instrument))this.kit.set(h.instrument,builtInDrum(this.ctx,h.instrument));s.buffer=this.samples.get(h.instrument)||this.kit.get(h.instrument);g.gain.value=(h.velocity/127)**1.5;s.connect(g);g.connect(output);const at=next+h.tick/960*60/config.bpm;s.start(at);if(h.instrument>=1&&h.instrument<=3){const choke=hits.find(x=>x.tick>h.tick&&x.instrument>=1&&x.instrument<=3);if(choke)s.stop(next+choke.tick/960*60/config.bpm);}this.sources.add(s);s.onended=()=>{this.sources.delete(s);g.disconnect();};}next+=duration;if(!loop){clearInterval(this.timer);return;}}};
    this.timer=setInterval(schedule,40);schedule();
  }
  close(){this.stop();this.ctx.close();}
}
