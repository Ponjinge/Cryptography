import logo from "./logo.svg";
import "./App.css";
import React, { useState } from "react";
import axios from "axios";

function App() {
  return (
    <div className="App">
      <CreateUserForm />
    </div>
  );
}

export default App;


function CreateUserForm() {
  const [name, setName] = useState('');

  const handleSubmit = async (event) => {
    event.preventDefault();
    try {
      const response = await axios.post('http://localhost:3030/user', {
        name,
      });
      console.log(response.data);
      alert('User created successfully!');
    } catch (error) {
      console.error('Error creating user:', error);
      alert('Failed to create user.');
    }
  };

  return (
    <div>
      <h2>Create User</h2>
      <form onSubmit={handleSubmit}>
        <label>
          Name:
          <input
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </label>
        <button type="submit">Submit</button>
      </form>
    </div>
  );
}

