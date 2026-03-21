import React from 'react';

export function Header() {
    return (
        <header className="p-4 bg-white border-b shadow-sm">
            <nav>
                <ul className="flex gap-6 list-none m-0 p-0">
                    <li><a href="/" className="text-blue-600 hover:underline font-medium">Index</a></li>
                    <li><a href="/home" className="text-blue-600 hover:underline font-medium">Home</a></li>
                    <li><a href="/api-client" className="text-blue-600 hover:underline font-medium">API Client</a></li>
                </ul>
            </nav>
        </header>
    )
}