import { useState } from "react";
export function LoginPage() {
    const [email ,setEmail] =useState('')
    const [password, setPassword] = useState('')

    async function handleLogin(e) {
    e.preventDefault()
    const response = await fetch("http://localhost:5000/api/auth/login",{
      method:"POST",
      headers:{
        "Content-Type":"application/json",
      },
      body:JSON.stringify({
        email,
        password
      }),
    })


    if (response.ok) {
      setMessage("Succesfully loged in")
      setEmail("")
      setPassword("")
      const data = await response.json()
      localStorage.setItem("token",data.token)
     
    }
    else{
      setMessage("Invalid username or password")
    }

    const token = localStorage.getItem("token")

    console.log(`Token : ${token}`)

  
  }
    
    return (
        <>
        <form action=""className="Login-form" onSubmit={handleLogin}>
        <label htmlFor="" className="email-label"> Enter Email</label>
        <input type="text" className="input-email" value={email} onSubmit={(e)=>setEmail(e.target.value)} />

        <label htmlFor="" className="pasword-label">Enter Password</label>
        <input type="text" className="password" 
        value={password} onChange={(e)=>setPassword(e.target.value)}
        />

        <button type="submit" className="Login-button">Login</button>

        </form>
        
        </>
    )
}