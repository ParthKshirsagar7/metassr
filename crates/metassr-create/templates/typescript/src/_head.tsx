import React from 'react';

export default function Head() {
    return (
        <>
            <meta charSet="UTF-8" />
            <meta name="description" content="%DESC%" />
            <meta name="keywords" content="MetaSSR, React, SSR, SSG" />
            <meta name="author" content="%NAME%" />
            <link rel="icon" type="image/png" href="/static/assets/metacall-logo.png" />
            <title> %NAME% | %VER% </title>
        </>
    );
}


