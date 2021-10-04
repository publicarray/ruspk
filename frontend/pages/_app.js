import 'tailwindcss/tailwind.css'
import { ThemeProvider } from 'next-themes'

function MyApp({ Component, pageProps }) {
    return (
        <ThemeProvider attribute="class" enableColorScheme="true">
            <Component {...pageProps} />
        </ThemeProvider>
    )
}
export default MyApp
