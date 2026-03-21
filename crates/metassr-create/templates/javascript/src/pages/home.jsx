import React, { useState } from 'react';
import metacalllogo from '../../static/assets/metacall-logo.png'

export default function Home() {
	let [counter, setCounter] = useState(0);

	return (
		<div className="flex flex-col items-center justify-center min-h-screen gap-6">
			<img src={metacalllogo} width="200px" alt="MetaCall logo" />

			<div className="text-4xl font-bold">This is a simple home page with a counter</div>

			<h1 className="text-4xl font-bold">{counter}</h1>
			<button
				className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
				onClick={() => { setCounter(counter + 1); }}
			>
				Click me
			</button>
		</div>
	)
}
