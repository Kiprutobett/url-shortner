import { useState } from 'react'
import { Login } from "./Pages/";
// api.js does not currently provide TypeScript declarations.
// @ts-expect-error -- the API shape is declared below.
import { api } from './api.js'



interface UrlData{
  short_code:String,
  id : String,
  original_url: String
}


function App() {
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [message,setMessage]=useState("")
  const [data, setData] = useState<UrlData[]>([])

  

  async function fetch_urls() {

    const token = localStorage.getItem("token")

    const response = await fetch("http://localhost:5000/api/urls",{
      headers:{
        Authorization: `Bearer ${token}`
      }
    }
    )

    if (response.ok) {
      const data =await response.json()
      console.log(data)
      setData(data)
    }
    else{
      setMessage("Unable to fetch data")
      throw new Error(`Fetch failed :${response.status}`);
      return
    }
    
  }

  async function submit_url() {
    const token =localStorage.getItem("token")

    const url_data = JSON.stringify({
      "url":"htts://github.com"
    })

    let rsponse = await fetch("http://localhost:5000/api/url",{
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
  }

  

  return (
    <>
    <form onSubmit={handleLogin}>
      <input 
      placeholder='email' 
        name="" 
        id=""
        value={email} 
        onChange={(e)=>setEmail(e.target.value)}/>
      <input 
        placeholder='password'
        type="password" 
        name="" 
        id=""
        value={password} 
        onChange={(e)=>setPassword(e.target.value)}/>
      
      <button type="submit">Login</button>
    </form>
    <h1>{message}</h1>

    <button type="button" onClick={()=>fetch_urls()}>fetch urls</button>

    <div className='list-containers'>
      {data.map((url)=>(
        <div className='url-container'>
          <h2>{url.id}</h2>
          <p>{url.original_url}</p>
          <p>Id:{url.id}</p>
        </div>
      ))}
    </div>

    <div className='submit-url'>


    </div>
    </>
  )
}

export default App
