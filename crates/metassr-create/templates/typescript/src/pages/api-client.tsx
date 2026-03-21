import React, { useState, FormEvent } from 'react';

export default function ApiClient() {
    const [response, setResponse] = useState<any>(null);
    const [name, setName] = useState<string>('');
    const [loading, setLoading] = useState<boolean>(false);

    const handleGet = async () => {
        setLoading(true);
        try {
            const res = await fetch('/api/hello');
            const data = await res.json();
            setResponse(data);
        } catch (err: any) {
            setResponse({ error: err.message });
        } finally {
            setLoading(false);
        }
    };

    const handlePost = async (e: FormEvent) => {
        e.preventDefault();
        setLoading(true);
        try {
            const res = await fetch('/api/hello', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(name ? { name: name } : {})
            });
            const data = await res.json();
            setResponse(data);
        } catch (err: any) {
            setResponse({ error: err.message });
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="flex flex-col items-center min-h-screen gap-8 p-8 max-w-3xl mx-auto">
            <h1 className="text-4xl font-bold">API Test Client</h1>
            
            <div className="grid grid-cols-1 md:grid-cols-2 gap-8 w-full">
                <div className="p-6 border rounded shadow-sm bg-white">
                    <h2 className="text-2xl font-semibold mb-4">GET Request</h2>
                    <p className="text-gray-600 mb-4 h-12">Fetch a welcome message and timestamp from the server.</p>
                    <button 
                        onClick={handleGet}
                        disabled={loading}
                        className="w-full px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition"
                    >
                        Send GET Request
                    </button>
                </div>

                <div className="p-6 border rounded shadow-sm bg-white">
                    <h2 className="text-2xl font-semibold mb-4">POST Request</h2>
                    <p className="text-gray-600 mb-4 h-12">Send data to the server and receive a personalized response.</p>
                    <form onSubmit={handlePost} className="flex flex-col gap-4">
                        <input 
                            type="text" 
                            value={name}
                            onChange={(e) => setName(e.target.value)}
                            placeholder="Enter your name" 
                            className="w-full px-3 py-2 border rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                        <button 
                            type="submit"
                            disabled={loading}
                            className="w-full px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700 transition"
                        >
                            Send POST Request
                        </button>
                    </form>
                </div>
            </div>

            <div className="w-full p-0 border rounded shadow-sm overflow-hidden flex flex-col mt-4">
                <div className="bg-gray-100 p-4 border-b">
                    <h2 className="text-xl font-semibold m-0">Response</h2>
                </div>
                <pre className="bg-gray-800 text-green-400 p-4 m-0 overflow-auto h-64 text-sm text-left">
                    {response ? JSON.stringify(response, null, 2) : 'No response yet. Send a request above!'}
                </pre>
            </div>
        </div>
    );
}
