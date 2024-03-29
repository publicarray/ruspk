import { useTheme } from 'next-themes'
import { useState, useEffect } from "react";

const ThemeChanger = () => {
    const [mounted, setMounted] = useState(false)
    const {theme, setTheme, forcedTheme, resolvedTheme, systemTheme, themes} = useTheme()

    // When mounted on client, now we can show the UI
    useEffect(() => setMounted(true), [])

    if (!mounted) return null

    return (
        <button
            aria-label="Toggle Dark Mode"
            type="button"
            className="flex mx-4"
            onClick={() => setTheme(resolvedTheme === 'dark' ? 'light' : 'dark')}
        >
            {resolvedTheme === 'light' &&
                <svg xmlns="http://www.w3.org/2000/svg" className="h-12 w-12 bg-gray-300 hover:bg-gray-400 p-2 rounded-xl" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
                </svg>
            }
            {resolvedTheme === 'dark' &&
                <svg xmlns="http://www.w3.org/2000/svg" className="h-12 w-12 bg-gray-600 hover:bg-gray-500 p-2 rounded-xl" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
                </svg>
            }
        </button>
    )
}

export default ThemeChanger;
