"use client";
import axios from "axios";
import { useState } from "react";


export default function Home() {

  const [response, setResponse] = useState();

  const backendTest = async () => {
    try {
      const res = await axios.get('http://localhost/api/');
      console.log(res.data);
      setResponse(res.data);
    } catch (error) {
      console.error('Error fetching data:', error);
    }
  }

  return (
    <div>
      <p>Hello!!</p>
      <button className="bg-amber-200 border p-2 m-2" onClick={backendTest}>BackendTest</button>
      <p>Response: {response}</p>
    </div>
  );
}
