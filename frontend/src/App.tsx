import { useState } from 'react';
import './App.css';
import UserWallet from "./components/UserWallet";

function App() {
  const [username, setUsername] = useState<string | null>(null);
  const [address, setAddress] = useState<string | null>(null);
  
  const handleLogout = () => {
    setUsername(null);
    setAddress(null);
  };

  return (
    <div className="app-container">
      <div className="card">
        {username && address ? (
          <>
            <h1>Bine ai venit, {username}!</h1>
            <p>Adresa wallet-ului tău: {address}</p>
            <p>Balans: 0.0 BTC</p>
            <button onClick={handleLogout} className="logout-btn">Logout</button>
          </>
        ) : (
          <UserWallet setUsername={setUsername} setAddress={setAddress} />
        )}
      </div>
    </div>
  );
}

export default App;
