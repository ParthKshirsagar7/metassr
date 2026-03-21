import React from 'react';

export default function NotFound() {
    return (
        <div className="flex flex-col items-center justify-center min-h-screen gap-4">
            <h1 className="text-4xl font-bold">404 — Page Not Found</h1>
            <p className="text-gray-600">The page you're looking for doesn't exist.</p>
            <a href='/'>
                <button className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">
                    Back Home
                </button>
            </a>
        </div>
    )
}
