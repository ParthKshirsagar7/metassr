import React from 'react';
import metacalllogo from '../../static/assets/metacall-logo.png'

export default function Index() {
	return (
		<div className='flex flex-col items-center justify-center min-h-screen gap-8'>
			<img src={metacalllogo} width="200px" alt="MetaCall logo" />
			<h1 className='text-4xl font-bold'>Welcome to MetaSSR</h1>
			<p className='text-lg text-gray-600'>Server-Side Rendering &amp; Static Site Generation with React</p>

			<div className='grid grid-cols-2 gap-6 mt-8'>
				<div className='p-4 border rounded shadow'>
					<h2 className='text-xl font-semibold mb-2'>Static-Site Generation</h2>
					<code className='text-sm bg-gray-100 px-2 py-1 rounded'>$ metassr build -t ssg</code>
				</div>
				<div className='p-4 border rounded shadow'>
					<h2 className='text-xl font-semibold mb-2'>Server-Side Rendering</h2>
					<code className='text-sm bg-gray-100 px-2 py-1 rounded'>$ metassr build -t ssr</code>
				</div>
				<div className='p-4 border rounded shadow col-span-2'>
					<h2 className='text-xl font-semibold mb-2'>Run your application</h2>
					<code className='text-sm bg-gray-100 px-2 py-1 rounded'>$ metassr run</code>
				</div>
			</div>
		</div>
	)
}
