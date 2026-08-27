(()=>{'use strict';
const manifest=JSON.parse(document.querySelector('#manifest').textContent);
const hash=document.querySelector('#bundle-meta').dataset.hash;
const artifactList=document.querySelector('#artifact-list');
const gapList=document.querySelector('#gap-list');
const storageKey=`khb:${hash}:reviewed`;
let reviewed=new Set();
try{reviewed=new Set(JSON.parse(localStorage.getItem(storageKey)||'[]'))}catch(_error){}
const artifacts=manifest.sections.flatMap((section)=>section.artifacts.map((artifact)=>({...artifact,section:section.title})));
const el=(tag,className,text)=>{const node=document.createElement(tag);if(className)node.className=className;if(text!==undefined)node.textContent=text;return node};
const addMeta=(dl,label,value)=>{if(!value)return;const box=el('div');box.append(el('dt','',label),el('dd','',value));dl.append(box)};
function renderArtifacts(filter='all'){
  artifactList.replaceChildren();
  const visible=artifacts.filter((a)=>filter==='all'||(filter==='required'&&a.required)||(filter==='attention'&&!['verified'].includes(a.status)));
  document.querySelector('#filter-empty').hidden=visible.length>0;
  visible.forEach((artifact,index)=>{
    const row=el('article','track');row.dataset.status=artifact.status;row.dataset.required=String(artifact.required);
    row.append(el('div','track-number',String(index+1).padStart(2,'0')));
    const body=el('div');const titleWrap=el('div','track-title');titleWrap.append(el('h3','',artifact.title));if(artifact.required)titleWrap.append(el('span','required','Required'));
    body.append(titleWrap);
    if(artifact.note)body.append(el('p','note',artifact.note));
    const meta=el('dl','meta');addMeta(meta,'Owner',artifact.owner);addMeta(meta,'Section',artifact.section);addMeta(meta,'Expiry',artifact.expires_at);addMeta(meta,'SHA-256',artifact.sha256?artifact.sha256.slice(0,16)+'…':null);body.append(meta);
    const actions=el('div','track-actions');actions.append(el('span',`stamp ${artifact.status}`,`${artifact.status} · ${artifact.status_detail}`));
    if(artifact.href){const link=el('a','artifact-link',artifact.kind==='file'?'Open copied file':'Open public link');link.href=artifact.href;if(artifact.kind==='url'){link.target='_blank';link.rel='noreferrer noopener'}actions.append(link)}
    const label=el('label','review');const checkbox=document.createElement('input');checkbox.type='checkbox';checkbox.checked=reviewed.has(artifact.id);checkbox.dataset.id=artifact.id;checkbox.addEventListener('change',()=>{checkbox.checked?reviewed.add(artifact.id):reviewed.delete(artifact.id);try{localStorage.setItem(storageKey,JSON.stringify([...reviewed]))}catch(_error){};});label.append(checkbox,document.createTextNode(' Reviewed'));actions.append(label);
    row.append(body,actions);artifactList.append(row);
  });
}
function renderGaps(){
  if(!manifest.gaps.length){gapList.append(el('p','empty','No unresolved gaps were declared. Confirm this with the departing owner.'));return}
  manifest.gaps.forEach((gap,index)=>{const row=el('article','gap');row.append(el('div','track-number',`G${index+1}`));const body=el('div');body.append(el('h3','',gap.title),el('p','',`Owner: ${gap.owner}`),el('p','',`Next step: ${gap.next_step}`));row.append(body);gapList.append(row)})
}
document.querySelectorAll('.filter').forEach((button)=>button.addEventListener('click',()=>{document.querySelectorAll('.filter').forEach((other)=>{const active=other===button;other.classList.toggle('active',active);other.setAttribute('aria-pressed',String(active))});renderArtifacts(button.dataset.filter)}));
document.querySelector('#export').addEventListener('click',()=>{const recipient=document.querySelector('#recipient').value.trim();const status=document.querySelector('#ack-status');if(!recipient){status.textContent='Enter the recipient name before exporting.';document.querySelector('#recipient').focus();return}const acknowledgement={format:'knowledge-handoff-acknowledgement/1',project:manifest.project.title,recipient,accepted:[...reviewed].sort(),note:document.querySelector('#ack-note').value.trim()||null,acknowledged_at:new Date().toISOString(),manifest_sha256:hash};const blob=new Blob([JSON.stringify(acknowledgement,null,2)+'\n'],{type:'application/json'});const link=document.createElement('a');link.href=URL.createObjectURL(blob);link.download='acknowledgement.json';link.click();setTimeout(()=>URL.revokeObjectURL(link.href),1000);status.textContent=`Exported receipt with ${reviewed.size} reviewed artifact${reviewed.size===1?'':'s'}.`});
const setNetwork=()=>document.body.classList.toggle('offline',!navigator.onLine);addEventListener('online',setNetwork);addEventListener('offline',setNetwork);setNetwork();renderArtifacts();renderGaps();
})();
