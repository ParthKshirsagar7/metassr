import React, { useState } from "react";

export function Footer() {
    const [counter, setCounter] = useState(0)
    return (
        <footer className="p-4 bg-gray-50 border-t text-center">
            <div>This is a footer</div>
            <button onClick={() => setCounter(counter + 1)}>
                This is a counter from footer {counter}
            </button>
        </footer>
    );
}

