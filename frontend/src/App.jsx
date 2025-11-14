import React, { useState, useEffect } from 'react';
import RoastCard from './components/RoastCard.jsx';
import RoastForm from './components/RoastForm.jsx';
import WalletConnect from './components/WalletConnect.jsx';
import { getRoastCount, getRoast, postRoast, upvoteRoast } from './lib/soroban.js';
import { SorobanClient } from '@stellar/soroban-client';

export default function App() {
  const [account, setAccount] = useState(null);
  const [roasts, setRoasts] = useState([]);
  const [target, setTarget] = useState('');
  const [content, setContent] = useState('');

  const CONTRACT_ID = import.meta.env.VITE_CONTRACT_ID;
  const RPC = import.meta.env.VITE_RPC_URL;

  const client = new SorobanClient(RPC);

  async function loadRoasts() {
    try {
      const count = await getRoastCount(client, CONTRACT_ID);
      const items = [];
      for (let i = 1; i <= count; i++) {
        const roast = await getRoast(client, CONTRACT_ID, i);
        items.push(roast);
      }
      setRoasts(items.reverse());
    } catch (err) {
      console.error('Failed loading roasts:', err);
    }
  }

  async function submitRoast() {
    if (!account) return alert('Connect wallet first');
    if (!content) return alert('Write a roast');
    try {
      await postRoast(client, CONTRACT_ID, account, target, content);
      setContent('');
      await loadRoasts();
    } catch (err) {
      console.error(err);
      alert('Failed posting roast');
    }
  }

  async function handleUpvote(id) {
    if (!account) return alert('Connect wallet first');
    try {
      await upvoteRoast(client, CONTRACT_ID, account, id);
      await loadRoasts();
    } catch (err) {
      console.error(err);
      alert('Failed upvoting');
    }
  }

  useEffect(() => {
    loadRoasts();
  }, []);

  return (
    <div className="container">
      <h1>🔥 On-Chain Roaster</h1>

      <WalletConnect onConnect={setAccount} />

      <RoastForm
        target={target}
        setTarget={setTarget}
        content={content}
        setContent={setContent}
        onSubmit={submitRoast}
      />

      <h2>Recent Roasts</h2>

      {roasts.length === 0 ? (
        <p>No roasts yet</p>
      ) : (
        roasts.map((r, i) => <RoastCard key={i} roast={r} onUpvote={handleUpvote} />)
      )}
    </div>
  );
}
