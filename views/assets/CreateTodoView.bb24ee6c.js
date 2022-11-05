import{d as T,I as b,B as r,S,a as g,x as B,s as p,A as y,_ as w,r as n,o as k,c as v,f as d,e as o,j as i,a0 as I,g as V,W as C,w as A,v as u,p as $,k as q}from"./index.679e4c1f.js";import{A as L}from"./AppEmptyState.03179617.js";import{u as c}from"./tasks.18199f5e.js";const R=T({name:"CreateNewTask",components:{Icon:b,BaseTextInput:r,AppEmptyState:L,Spinner:S,BaseButton:g,BaseTextarea:r},data:()=>({showTodoModal:!1,todo:{title:"",description:"",date:"",priority:""}}),mounted(){console.log("mounted")},created(){this.makeTodoRequest(),console.log("created hook")},methods:{...B(c,{makeTodoRequest:"fetchAllTodo",createTodo:"createTodo",deleteTodo:"deleteTodo"}),async makeCreateTodo(){await this.createTodo({...this.todo})&&(Object.assign(this.todo,{}),this.$router.replace({name:"all-task"}))}},computed:{...p(c,["todoArray","isLoading","isFetchingTodoArray","errorFetchingTodo"]),...p(y,{bearerToken:"authorizationToken"}),disabledState(){return this.isLoading===!0}}});const a=e=>($("data-v-e09e6b9f"),e=e(),q(),e),z={class:"select__form__field"},M=a(()=>o("label",{for:"priority"},"Priority",-1)),N=a(()=>o("option",{disabled:"",value:""},"Please select priority",-1)),U=a(()=>o("option",null,"urgent",-1)),j=a(()=>o("option",null,"not urgent",-1)),E=a(()=>o("option",null,"important",-1)),F=a(()=>o("option",null,"delicate",-1)),P=a(()=>o("option",null,"normal",-1)),x=[N,U,j,E,F,P];function D(e,t,O,W,G,H){const l=n("BaseTextInput"),m=n("BaseTextarea"),_=n("Icon"),f=n("Spinner"),h=n("BaseButton");return k(),v("form",{action:"",onSubmit:t[3]||(t[3]=A((...s)=>e.makeCreateTodo&&e.makeCreateTodo(...s),["prevent"]))},[d(l,{label:"heading",type:"text",placeholder:"heading",modelValue:e.todo.title,"onUpdate:modelValue":t[0]||(t[0]=s=>e.todo.title=s),class:"field"},null,8,["modelValue"]),d(m,{placeholder:"description",label:"description",modelValue:e.todo.description,"onUpdate:modelValue":t[1]||(t[1]=s=>e.todo.description=s),class:"field"},null,8,["modelValue"]),o("div",z,[M,i(o("select",{"onUpdate:modelValue":t[2]||(t[2]=s=>e.todo.priority=s)},x,512),[[I,e.todo.priority]]),d(_,{icon:"mdi:menu-down",class:"select__arrow__down"})]),d(l,{placeholder:"github url",label:"due date",type:"date",model:e.todo.date,class:"field"},null,8,["model"]),d(h,{text:"",type:"submit",disabled:e.disabledState,class:C([e.disabledState==!0?"disabled__button":""])},{default:V(()=>[i(o("span",null,"Add Todo",512),[[u,!e.isLoading]]),i(d(f,{"animation-duration":1e3,size:30,color:"#ffffff"},null,512),[[u,e.isLoading]])]),_:1},8,["disabled","class"])],32)}const X=w(R,[["render",D],["__scopeId","data-v-e09e6b9f"]]);export{X as default};
