
            let ns,bubbles,ptr,id,event_name,value,field,len;
            export class JSChannel {
                constructor(r) {
                    this.d=r;
                    this.m = null;
                    this.p = null;
                    this.ls = null;
                    this.t = null;
                    this.op = null;
                    this.e = null;
                    this.z = null;
                    this.metaflags = null;
                    this.u32buf=null;this.u32bufp=null;this.evt = [];
                    this.evt_cache_hit = null;
                    this.evt_cache_idx;
                    this.get_evt = function() {
                        this.evt_cache_idx = this.u8buf[this.u8bufp++];
                        if(this.evt_cache_idx & 128){
                            this.evt_cache_hit=this.s.substring(this.sp,this.sp+=this.u8buf[this.u8bufp++]);
                            this.evt[this.evt_cache_idx&4294967167]=this.evt_cache_hit;
                            return this.evt_cache_hit;
                        }
                        else{
                            return this.evt[this.evt_cache_idx&4294967167];
                        }
                    };this.attr = [];
                    this.attr_cache_hit = null;
                    this.attr_cache_idx;
                    this.get_attr = function() {
                        this.attr_cache_idx = this.u8buf[this.u8bufp++];
                        if(this.attr_cache_idx & 128){
                            this.attr_cache_hit=this.s.substring(this.sp,this.sp+=this.u8buf[this.u8bufp++]);
                            this.attr[this.attr_cache_idx&4294967167]=this.attr_cache_hit;
                            return this.attr_cache_hit;
                        }
                        else{
                            return this.attr[this.attr_cache_idx&4294967167];
                        }
                    };this.u16buf=null;this.u16bufp=null;this.u8buf=null;this.u8bufp=null;this.ns_cache = [];
                    this.ns_cache_cache_hit = null;
                    this.ns_cache_cache_idx;
                    this.get_ns_cache = function() {
                        this.ns_cache_cache_idx = this.u8buf[this.u8bufp++];
                        if(this.ns_cache_cache_idx & 128){
                            this.ns_cache_cache_hit=this.s.substring(this.sp,this.sp+=this.u8buf[this.u8bufp++]);
                            this.ns_cache[this.ns_cache_cache_idx&4294967167]=this.ns_cache_cache_hit;
                            return this.ns_cache_cache_hit;
                        }
                        else{
                            return this.ns_cache[this.ns_cache_cache_idx&4294967167];
                        }
                    };this.s = "";this.lsp = null;this.sp = null;this.sl = null;this.c = new TextDecoder();
                    this.setAttributeInner = function (node, field, value, ns) {
  const name = field;
  if (ns === "style") {
    // ????? why do we need to do this
    if (node.style === undefined) {
      node.style = {};
    }
    node.style[name] = value;
  } else if (!!ns) {
    node.setAttributeNS(ns, name, value);
  } else {
    switch (name) {
      case "value":
        if (value !== node.value) {
          node.value = value;
        }
        break;
      case "initial_value":
        node.defaultValue = value;
        break;
      case "checked":
        node.checked = truthy(value);
        break;
      case "initial_checked":
        node.defaultChecked = truthy(value);
        break;
      case "selected":
        node.selected = truthy(value);
        break;
      case "initial_selected":
        node.defaultSelected = truthy(value);
        break;
      case "dangerous_inner_html":
        node.innerHTML = value;
        break;
      default:
        // https://github.com/facebook/react/blob/8b88ac2592c5f555f315f9440cbb665dd1e7457a/packages/react-dom/src/shared/DOMProperty.js#L352-L364
        if (!truthy(value) && bool_attrs.hasOwnProperty(name)) {
          node.removeAttribute(name);
        } else {
          node.setAttribute(name, value);
        }
    }
  }
}

const bool_attrs = {
  allowfullscreen: true,
  allowpaymentrequest: true,
  async: true,
  autofocus: true,
  autoplay: true,
  checked: true,
  controls: true,
  default: true,
  defer: true,
  disabled: true,
  formnovalidate: true,
  hidden: true,
  ismap: true,
  itemscope: true,
  loop: true,
  multiple: true,
  muted: true,
  nomodule: true,
  novalidate: true,
  open: true,
  playsinline: true,
  readonly: true,
  required: true,
  reversed: true,
  selected: true,
  truespeed: true,
  webkitdirectory: true,
};

function truthy(val) {
  return val === "true" || val === true;
}

    class ListenerMap {
        constructor(root) {
            // bubbling events can listen at the root element
            this.global = {};
            // non bubbling events listen at the element the listener was created at
            this.local = {};
            this.root = root;
            this.handler = null;
        }

        create(event_name, element, bubbles) {
            if (bubbles) {
                if (this.global[event_name] === undefined) {
                    this.global[event_name] = {};
                    this.global[event_name].active = 1;
                    this.root.addEventListener(event_name, this.handler);
                } else {
                    this.global[event_name].active++;
                }
            }
            else {
                const id = element.getAttribute("data-dioxus-id");
                if (!this.local[id]) {
                    this.local[id] = {};
                }
                element.addEventListener(event_name, this.handler);
            }
        }

        remove(element, event_name, bubbles) {
            if (bubbles) {
                this.global[event_name].active--;
                if (this.global[event_name].active === 0) {
                    this.root.removeEventListener(event_name, this.global[event_name].callback);
                    delete this.global[event_name];
                }
            }
            else {
                const id = element.getAttribute("data-dioxus-id");
                delete this.local[id][event_name];
                if (this.local[id].length === 0) {
                    delete this.local[id];
                }
                element.removeEventListener(event_name, this.handler);
            }
        }

        removeAllNonBubbling(element) {
            const id = element.getAttribute("data-dioxus-id");
            delete this.local[id];
        }
    }
    this.LoadChild = function(ptr, len) {
        // iterate through each number and get that child
        let node = this.stack[this.stack.length - 1];
        let ptr_end = ptr + len;
        for (; ptr < ptr_end; ptr++) {
            let end = this.m.getUint8(ptr);
            for (node = node.firstChild; end > 0; end--) {
                node = node.nextSibling;
            }
        }
        return node;
    }
    this.listeners = new ListenerMap();
    this.nodes = [];
    this.stack = [];
    this.root = null;
    this.templates = {};
    this.els = null;
    this.save_template = function(nodes, tmpl_id) {
        this.templates[tmpl_id] = nodes;
    }
    this.hydrate = function (ids) {
        const hydrateNodes = document.querySelectorAll('[data-node-hydration]');
        for (let i = 0; i < hydrateNodes.length; i++) {
            const hydrateNode = hydrateNodes[i];
            const hydration = hydrateNode.getAttribute('data-node-hydration');
            const split = hydration.split(',');
            const id = ids[parseInt(split[0])];
            this.nodes[id] = hydrateNode;
            if (split.length > 1) {
                hydrateNode.listening = split.length - 1;
                hydrateNode.setAttribute('data-dioxus-id', id);
                for (let j = 1; j < split.length; j++) {
                    const listener = split[j];
                    const split2 = listener.split(':');
                    const event_name = split2[0];
                    const bubbles = split2[1] === '1';
                    this.listeners.create(event_name, hydrateNode, bubbles);
                }
            }
        }
        const treeWalker = document.createTreeWalker(
            document.body,
            NodeFilter.SHOW_COMMENT,
        );
        let currentNode = treeWalker.nextNode();
        while (currentNode) {
            const id = currentNode.textContent;
            const split = id.split('node-id');
            if (split.length > 1) {
                this.nodes[ids[parseInt(split[1])]] = currentNode.nextSibling;
            }
            currentNode = treeWalker.nextNode();
        }
    }
    this.get_node = function(id) {
        return this.nodes[id];
    }
    this.initialize = function(root, handler) {
        this.listeners.handler = handler;
        this.nodes = [root];
        this.stack = [root];
        this.listeners.root = root;
    }
    this.AppendChildren = function (id, many){
        let root = this.nodes[id];
        this.els = this.stack.splice(this.stack.length-many);
        for (let k = 0; k < many; k++) {
            root.appendChild(this.els[k]);
        }
    }
    
                }

                update_memory(b){
                    this.m=new DataView(b.buffer)
                }

                run(){
                    this.metaflags=this.m.getUint32(this.d,true);
                    if((this.metaflags>>>6)&1){
                        this.ls=this.m.getUint32(this.d+6*4,true);
                    }
                    this.p=this.ls;
                    if ((this.metaflags>>>3)&1){
                this.t = this.m.getUint32(this.d+3*4,true);
                this.u32buf=new Uint32Array(this.m.buffer,this.t,((this.m.buffer.byteLength-this.t)-(this.m.buffer.byteLength-this.t)%4)/4);
            }
            this.u32bufp=0;if ((this.metaflags>>>4)&1){
                this.t = this.m.getUint32(this.d+4*4,true);
                this.u16buf=new Uint16Array(this.m.buffer,this.t,((this.m.buffer.byteLength-this.t)-(this.m.buffer.byteLength-this.t)%2)/2);
            }
            this.u16bufp=0;if ((this.metaflags>>>5)&1){
                this.t = this.m.getUint32(this.d+5*4,true);
                this.u8buf=new Uint8Array(this.m.buffer,this.t,((this.m.buffer.byteLength-this.t)-(this.m.buffer.byteLength-this.t)%1)/1);
            }
            this.u8bufp=0;if (this.metaflags&1){
                this.lsp = this.m.getUint32(this.d+1*4,true);
            }
            if ((this.metaflags>>>2)&1) {
                this.sl = this.m.getUint32(this.d+2*4,true);
                if ((this.metaflags>>>1)&1) {
                    this.sp = this.lsp;
                    this.s = "";
                    this.e = this.sp + ((this.sl / 4) | 0) * 4;
                    while (this.sp < this.e) {
                        this.t = this.m.getUint32(this.sp, true);
                        this.s += String.fromCharCode(
                            this.t & 255,
                            (this.t & 65280) >> 8,
                            (this.t & 16711680) >> 16,
                            this.t >> 24
                        );
                        this.sp += 4;
                    }
                    while (this.sp < this.lsp + this.sl) {
                        this.s += String.fromCharCode(this.m.getUint8(this.sp++));
                    }
                } else {
                    this.s = this.c.decode(new DataView(this.m.buffer, this.lsp, this.sl));
                }
            }
            this.sp=0;
                    for(;;){
                        this.op=this.m.getUint32(this.p,true);
                        this.p+=4;
                        this.z=0;
                        while(this.z++<4){
                            switch(this.op&255){
                                case 0:{this.AppendChildren(this.root, this.stack.length-1);}break;case 1:{this.stack.push(this.nodes[this.u32buf[this.u32bufp++]]);}break;case 2:{this.AppendChildren(this.u32buf[this.u32bufp++], this.u16buf[this.u16bufp++]);}break;case 3:{this.stack.pop();}break;case 4:{const root = this.nodes[this.u32buf[this.u32bufp++]]; this.els = this.stack.splice(this.stack.length-this.u16buf[this.u16bufp++]); if (root.listening) { this.listeners.removeAllNonBubbling(root); } root.replaceWith(...this.els);}break;case 5:{this.nodes[this.u32buf[this.u32bufp++]].after(...this.stack.splice(this.stack.length-this.u16buf[this.u16bufp++]));}break;case 6:{this.nodes[this.u32buf[this.u32bufp++]].before(...this.stack.splice(this.stack.length-this.u16buf[this.u16bufp++]));}break;case 7:{let node = this.nodes[this.u32buf[this.u32bufp++]]; if (node !== undefined) { if (node.listening) { this.listeners.removeAllNonBubbling(node); } node.remove(); }}break;case 8:{this.stack.push(document.createTextNode(this.s.substring(this.sp,this.sp+=this.u32buf[this.u32bufp++])));}break;case 9:{let node = document.createTextNode(this.s.substring(this.sp,this.sp+=this.u32buf[this.u32bufp++])); this.nodes[this.u32buf[this.u32bufp++]] = node; this.stack.push(node);}break;case 10:{let node = document.createElement('pre'); node.hidden = true; this.stack.push(node); this.nodes[this.u32buf[this.u32bufp++]] = node;}break;case 11:event_name=this.get_evt();id=this.u32buf[this.u32bufp++];bubbles=this.u8buf[this.u8bufp++];let node = this.nodes[id]; if(node.listening){node.listening += 1;}else{node.listening = 1;} node.setAttribute('data-dioxus-id', `${id}`); this.listeners.create(event_name, node, bubbles);break;case 12:{let node = this.nodes[this.u32buf[this.u32bufp++]]; node.listening -= 1; node.removeAttribute('data-dioxus-id'); this.listeners.remove(node, this.get_evt(), this.u8buf[this.u8bufp++]);}break;case 13:{this.nodes[this.u32buf[this.u32bufp++]].textContent = this.s.substring(this.sp,this.sp+=this.u32buf[this.u32bufp++]);}break;case 14:{let node = this.nodes[this.u32buf[this.u32bufp++]]; this.setAttributeInner(node, this.get_attr(), this.s.substring(this.sp,this.sp+=this.u32buf[this.u32bufp++]), this.get_ns_cache());}break;case 15:id=this.u32buf[this.u32bufp++];field=this.get_attr();ns=this.get_ns_cache();{
            let node = this.nodes[id];
            if (!ns) {
                switch (field) {
                    case "value":
                        node.value = "";
                        break;
                    case "checked":
                        node.checked = false;
                        break;
                    case "selected":
                        node.selected = false;
                        break;
                    case "dangerous_inner_html":
                        node.innerHTML = "";
                        break;
                    default:
                        node.removeAttribute(field);
                        break;
                }
            } else if (ns == "style") {
                node.style.removeProperty(name);
            } else {
                node.removeAttributeNS(ns, field);
            }
        }break;case 16:{this.nodes[this.u32buf[this.u32bufp++]] = this.LoadChild(this.u32buf[this.u32bufp++], this.u8buf[this.u8bufp++]);}break;case 17:ptr=this.u32buf[this.u32bufp++];len=this.u8buf[this.u8bufp++];value=this.s.substring(this.sp,this.sp+=this.u32buf[this.u32bufp++]);id=this.u32buf[this.u32bufp++];{
            let node = this.LoadChild(ptr, len);
            if (node.nodeType == node.TEXT_NODE) {
                node.textContent = value;
            } else {
                let text = document.createTextNode(value);
                node.replaceWith(text);
                node = text;
            }
            this.nodes[id] = node;
        }break;case 18:{this.els = this.stack.splice(this.stack.length - this.u16buf[this.u16bufp++]); let node = this.LoadChild(this.u32buf[this.u32bufp++], this.u8buf[this.u8bufp++]); node.replaceWith(...this.els);}break;case 19:{let node = this.templates[this.u16buf[this.u16bufp++]][this.u16buf[this.u16bufp++]].cloneNode(true); this.nodes[this.u32buf[this.u32bufp++]] = node; this.stack.push(node);}break;case 20:return true;
                            }
                            this.op>>>=8;
                        }
                    }
                }

                run_from_bytes(bytes){
                    this.d = 0;
                    this.update_memory(new Uint8Array(bytes))
                    this.run()
                }
            }