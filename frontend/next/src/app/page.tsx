"use client";
import axios from "axios";
import { useState } from "react";


export default function Home() {

  const [helloResponse, setHelloResponse] = useState();
  const [addUserResponse, setAddUserResponse] = useState();
  const [userId, setUserId] = useState<string>('');
  const [user, setUser] = useState();
  const [users, setUsers] = useState();

  const getHello = async () => {
    const res = await axios.get('/api/');
    setHelloResponse(res.data);
  }

  const postAddUser = async () => {
    const res = await axios.post('/api/users',
      {
        name: 'Bob',
        email: "naaabbbbaa"
      },
      {
        headers: {
          'API-KEY': "MTgpZPAnSnxxqZzFGVfj44aF-MwFes8LG4xv9Ubp"
        }
      })
    setAddUserResponse(res.data);
  }

  const getUser = async () => {
    const res = await axios.get(`/api/users/${userId}`,
      {
        headers: {
          'API-KEY': "MTgpZPAnSnxxqZzFGVfj44aF-MwFes8LG4xv9Ubp"
        }
      })
    setUser(res.data);
  }

  const getUsers = async () => {
    const res = await axios.get('/api/users',
      {
        headers: {
          'API-KEY': "MTgpZPAnSnxxqZzFGVfj44aF-MwFes8LG4xv9Ubp"
        }
      })
    setUsers(res.data);
  }

  return (
    <div className="px-4">
      <header className="flex items-center justify-between py-4">
        <h1 className="text-3xl font-bold underline">
          Hackathon Demo Application
        </h1>
      </header>
      <main className="flex flex-col gap-4">
        <div>
          <h1 className="text-2xl">1. Hello World</h1>
          <button className="bg-amber-200 border rounded-md p-2" onClick={getHello}>Send</button>
          {helloResponse && (
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded">
              {JSON.stringify(helloResponse, null, 2)}
            </pre>
          )}
        </div>
        <div>
          <h1 className="text-2xl">2. Add users</h1>
          <button className="bg-amber-200 border rounded-md p-2" onClick={postAddUser}>Send</button>
          {addUserResponse && (
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded">
              {JSON.stringify(addUserResponse, null, 2)}
            </pre>
          )}
        </div>
        <div>
          <h1 className="text-2xl">3. Get user</h1>
          <input
            type="text"
            className="border rounded-md p-2"
            placeholder="User Id"
            value={userId}
            onChange={(e) => setUserId(e.target.value)}

          />
          <button className="bg-amber-200 border rounded-md p-2" onClick={getUser}>Send</button>
          {user && (
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded">
              {JSON.stringify(user, null, 2)}
            </pre>
          )}
        </div>

        <div>
          <h1 className="text-2xl">4. Get users</h1>
          <button className="bg-amber-200 border rounded-md p-2" onClick={getUsers}>Send</button>
          {users && (
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded">
              {JSON.stringify(users, null, 2)}
            </pre>
          )}
        </div>
      </main>
    </div>
  );
}
