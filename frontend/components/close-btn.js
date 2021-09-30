export default function Button({className, children, onClick}) {
    return (
        <button onClick={onClick} className={`${className} py-3 px-6
        text-white rounded-lg bg-gray-500 hover:bg-gray-700 dark:bg-gray-500 dark:hover:bg-gray-700 focus:ring-blue-600
        shadow-lg block md:inline-block focus:outline-none focus:ring-2 focus:ring-opacity-50`}>{children}</button>
    );
}
