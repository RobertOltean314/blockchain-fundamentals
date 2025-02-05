import { useEffect, useState } from 'react';

interface Wallet {
  name: string;
  address: string;
  balance: number;
}

export default function WalletPage() {
  const [wallet, setWallet] = useState<Wallet | null>(null);
  const [username, setUsername] = useState('');
  const [loading, setLoading] = useState(false);

  // Verificăm dacă există un wallet salvat în localStorage
  useEffect(() => {
    const storedWallet = localStorage.getItem('wallet');
    if (storedWallet) {
      setWallet(JSON.parse(storedWallet));
    }
  }, []);

  // Funcția de creare a wallet-ului
  const handleCreateWallet = async () => {
    if (!username) {
      alert('Introdu un nume de utilizator!');
      return;
    }
  
    setLoading(true);
  
    try {
      const response = await fetch('/wallet/create', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username }),
      });
  
      if (!response.ok) {
        throw new Error('Eroare la crearea wallet-ului.');
      }
  
      const newWallet = await response.json();
      
      // Log pentru a verifica datele primite de la server
      console.log('Date primite de la server:', newWallet);
  
      localStorage.setItem('wallet', JSON.stringify(newWallet));
      setWallet(newWallet);
    } catch (error) {
      console.error('Eroare:', error);
      alert('A apărut o problemă la crearea wallet-ului.');
    } finally {
      setLoading(false);
    }
  };
  

  // Dacă există un wallet, afișăm detaliile
  if (wallet) {
    return (
      <div className="p-4">
        <h1 className="text-2xl font-bold">Bine ai venit, {wallet.name}!</h1>
        <p>Adresa wallet-ului tău: <span className="font-mono">{wallet.address}</span></p>
        <p>Balans: {wallet.balance} BTC</p>
      </div>
    );
  }

  // Dacă NU există un wallet, afișăm formularul de creare
  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold">Creează-ți un Wallet</h1>
      <input
        type="text"
        placeholder="Introdu numele de utilizator"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        className="border p-2 my-2 w-full rounded"
      />
      <button
        onClick={handleCreateWallet}
        disabled={loading}
        className="bg-blue-500 text-white p-2 rounded w-full"
      >
        {loading ? 'Se creează...' : 'Creează Wallet'}
      </button>
    </div>
  );
}
