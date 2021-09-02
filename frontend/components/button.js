export default function Button({children}) {
    return (
        <button className="py-3 px-6 text-white rounded-lg bg-blue-500 shadow-lg block md:inline-block hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-opacity-50">{children}</button>
    );
}

//py-3 px-6 text-white rounded-lg bg-blue-500 shadow-lg block md:inline-block hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-opacity-50

//transition duration-200 bg-blue-500 hover:bg-blue-600 focus:bg-blue-700 focus:shadow-sm focus:ring-4 focus:ring-blue-500 focus:ring-opacity-50 text-white w-full py-2.5 rounded-lg text-sm shadow-sm hover:shadow-md font-semibold text-center inline-block
