import { LiveKitRoom, VideoConference } from "@livekit/components-react";
import { fetchToken } from "./livekitToken";
import { useState } from "react";
import "./App.css";

export default function App() {
  const [token, setToken] = useState<string | null>(null);
  const [serverUrl] = useState("wss://test-8rb3uabn.livekit.cloud");

  async function connectToRoom() {
    try {
      const room = "test-room";
      const identity = "tauri-user";
      const t = await fetchToken(room, identity);
      setToken(t);
    } catch (err) {
      console.error("token error", err);
      alert("Failed to get token. Check Tauri logs.");
    }
  }

  return (
    <div className="h-screen flex flex-col">
      <header className="p-4 bg-gray-800 flex justify-between items-center">
        <h1 className="text-lg font-bold">LiveKit Test</h1>
        {!token && (
          <button
            onClick={connectToRoom}
            className="px-4 py-2 bg-blue-500 rounded hover:bg-blue-600"
          >
            Connect
          </button>
        )}
      </header>

      {token && (
        <LiveKitRoom
          token={token}
          serverUrl={serverUrl}
          connect={true}
          audio={true}
          video={true}
          className="flex-1 bg-gray-900"
        >
          <VideoConference />
        </LiveKitRoom>
      )}
    </div>
  );
}
