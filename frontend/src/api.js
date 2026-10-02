const BASE_URL ="http://localhost:5000"

async function request(path,options={}) {
    const res = await fetch(`${BASE_URL}${path}`,{...options,credentials:"include",headers:{"Content-Type":"aplication/json",...options.headers},})

    console.log(res)

        if (!res.ok) {
        const text = await res.text();
        throw new Error(text||`Request failed: ${res.status}`);
        
    }

    if (res.status === 204) return null;

    return res.json().catch(()=>null);
}

export const api ={
    register:(email,password)=>request("/api/auth/register",{
       method:"POST",body:JSON.stringify({email,password})
    }),

    login:(email,password)=>request("/api/auth/login",{
        method:"POST",body:JSON.stringify({email,password})
    })
}