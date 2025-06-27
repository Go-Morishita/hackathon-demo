"use client";
import axios from "axios";
import { useState } from "react";


export default function Home() {

  const [helloResponse, setHelloResponse] = useState();

  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [addUserResponse, setAddUserResponse] = useState();

  const [getUserId, setGetUserId] = useState<string>('');
  const [user, setUser] = useState();

  const [users, setUsers] = useState();

  const [putUserId, setPutUserId] = useState('');
  const [putUserName, setPutUserName] = useState('');
  const [putUserEmail, setPutUserEamil] = useState('');
  const [putUserResponse, setPutUserResponse] = useState();

  const [deleteUserId, setDeleteUserId] = useState('');
  const [deleteUserResponse, setDeleteUserResponse] = useState();


  const getHello = async () => {
    const res = await axios.get('/api/');
    setHelloResponse(res.data);
  }

  const postAddUser = async () => {
    const res = await axios.post('/api/users',
      {
        name: name,
        email: email
      },
      {
        headers: {
          'API-KEY': "MTgpZPAnSnxxqZzFGVfj44aF-MwFes8LG4xv9Ubp"
        }
      })
    setAddUserResponse(res.data);
  }

  const getUser = async () => {
    const res = await axios.get(`/api/users/${getUserId}`,
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

  const putUser = async () => {
    const res = await axios.put(`/api/users/${putUserId}`,
      {
        name: putUserName,
        email: putUserEmail
      },
      {
        headers: {
          'API-KEY': "MTgpZPAnSnxxqZzFGVfj44aF-MwFes8LG4xv9Ubp"
        }
      });
    setPutUserResponse(res.data);
  }

  const deleteUser = async () => {
    const res = await axios.delete(`/api/users/${deleteUserId}`,
      {
        headers: {
          'API-KEY': "MTgpZPAnSnxxqZzFGVfj44aF-MwFes8LG4xv9Ubp"
        }
      });
    setDeleteUserResponse(res.data);
  }

  return (
    <div className="px-4 pb-4">
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
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded mt-2">
              {JSON.stringify(helloResponse, null, 2)}
            </pre>
          )}
        </div>
        <div>
          <h1 className="text-2xl">2. User Table Schema</h1>
          <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded text-sm">
            {`CREATE TABLE users (
            id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE
) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;`}
          </pre>
        </div>

        <div>
          <h1 className="text-2xl">3. Post users</h1>
          <input
            type="text"
            className="border rounded-md p-2 me-1"
            placeholder="name"
            value={name}
            onChange={(e) => setName(e.target.value)}

          />
          <input
            type="text"
            className="border rounded-md p-2 me-1"
            placeholder="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}

          />
          <button className="bg-amber-200 border rounded-md p-2" onClick={postAddUser}>Send</button>
          {addUserResponse && (
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded mt-2">
              {JSON.stringify(addUserResponse, null, 2)}
            </pre>
          )}
        </div>
        <div>
          <h1 className="text-2xl">4. Get user</h1>
          <input
            type="text"
            className="border rounded-md p-2 me-1"
            placeholder="id"
            value={getUserId}
            onChange={(e) => setGetUserId(e.target.value)}

          />
          <button className="bg-amber-200 border rounded-md p-2" onClick={getUser}>Send</button>
          {user && (
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded mt-2">
              {JSON.stringify(user, null, 2)}
            </pre>
          )}
        </div>
        <div>
          <h1 className="text-2xl">5. Get users</h1>
          <button className="bg-amber-200 border rounded-md p-2" onClick={getUsers}>Send</button>
          {users && (
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded">
              {JSON.stringify(users, null, 2)}
            </pre>
          )}
        </div>
        <div>
          <h1 className="text-2xl">6. Put user</h1>
          <input
            type="text"
            className="border rounded-md p-2 me-1"
            placeholder="id"
            value={putUserId}
            onChange={(e) => setPutUserId(e.target.value)}
          />
          <input
            type="text"
            className="border rounded-md p-2 me-1"
            placeholder="name"
            value={putUserName}
            onChange={(e) => setPutUserName(e.target.value)}
          />
          <input
            type="text"
            className="border rounded-md p-2 me-1"
            placeholder="email"
            value={putUserEmail}
            onChange={(e) => setPutUserEamil(e.target.value)}
          />
          <button className="bg-amber-200 border rounded-md p-2" onClick={putUser}>Send</button>
          {putUserResponse && (
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded mt-2">
              {JSON.stringify(putUserResponse, null, 2)}
            </pre>
          )}
        </div>
        <div>
          <h1 className="text-2xl">7. Delete user</h1>
          <input
            type="text"
            className="border rounded-md p-2 me-1"
            placeholder="id"
            value={deleteUserId}
            onChange={(e) => setDeleteUserId(e.target.value)}
          />
          <button className="bg-amber-200 border rounded-md p-2 mt-2" onClick={deleteUser}>Send</button>
          {deleteUserResponse && (
            <pre className="whitespace-pre-wrap bg-gray-100 p-2 rounded mt-2">
              {JSON.stringify(deleteUserResponse, null, 2)}
            </pre>
          )}
        </div>
      </main>
    </div>
  );
}
