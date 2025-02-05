import React, { useState } from 'react';

interface UserWalletProps {
  setUsername: React.Dispatch<React.SetStateAction<string | null>>;
  setAddress: React.Dispatch<React.SetStateAction<string | null>>;
}

const UserWallet: React.FC<UserWalletProps> = ({ setUsername, setAddress }) => {
  const [usernameInput, setUsernameInput] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    const response = await fetch('http://localhost:3000/wallet/create', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username: usernameInput }),
    });

    const data = await response.json();
    if (data.address) {
      setUsername(usernameInput);
      setAddress(data.address);
    }
  };

  return (
    <div>
      <h2>Crează un wallet</h2>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Introduceti un nume"
          value={usernameInput}
          onChange={(e) => setUsernameInput(e.target.value)}
          required
        />
        <button type="submit">Creează Wallet</button>
      </form>
    </div>
  );
};

export default UserWallet;
