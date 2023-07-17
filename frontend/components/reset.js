import Link from "next/link";
import React, { useState } from "react";
import {postJsonForm, API} from '../utils';

export default function Reset({ errorMessage, setErrorMessage, onSubmit, processing, token}) {
    const [isOpen, setIsOpen] = useState(false);
    const [message, setMessage] = useState("");

    async function resetPassword (event) {
        console.log(event);
        event.preventDefault();
        const response = await postJsonForm(`${API}/reset`, event, []);
        console.log(response);
        setIsOpen(false);
        if (response) {
            window.location.replace('/reset');
            setMessage("Message send, Please check your inbox")
        } else {
            setErrorMessage("Error, Please try again later.")
        }
    }

    return (
        <div className="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col justify-center sm:py-12">
        <div className="p-10 xs:p-0 mx-auto md:w-full md:max-w-md">
            {/* <h1 className="font-bold text-center text-2xl mb-5">Your Logo</h1> */}
            <div className="bg-white shadow w-full rounded-lg divide-y divide-gray-200">
            <form onSubmit={onSubmit} className="px-5 py-7 text-gray-600">
                <input type="hidden" name="token" value={token} />
                <label className="font-semibold text-sm pb-1 block">New Password</label>
                <input type="password" name="password" className=" border rounded-lg px-3 py-2 mt-1 mb-5 text-sm w-full" />
                <label className="font-semibold text-sm pb-1 block">Confirm Password</label>
                <input type="password" name="confirm_password" className="border rounded-lg px-3 py-2 mt-1 mb-5 text-sm w-full" />
                {errorMessage && <p className="text-red-500 font-bold pb-1">{errorMessage}</p>}
                {message && <p className="font-bold pb-1">{message}</p>}
                <button id="login-btn" type="submit" className="transition duration-200 bg-blue-500 hover:bg-blue-600 focus:bg-blue-700 focus:shadow-sm focus:ring-4 focus:ring-blue-500 focus:ring-opacity-50 text-white w-full py-2.5 rounded-lg text-sm shadow-sm hover:shadow-md font-semibold text-center inline-block">
                    {processing
                    ? <div className="inline-block h-5 w-5 mx-3 border-t-transparent border-solid animate-spin rounded-full border-white border-4"></div>
                    : <>
                        <span className="inline-block mr-2">Set new Password</span>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" className="w-4 h-4 inline-block">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                        </svg>
                    </>
                    }
                </button>
            </form>
            </div>
            <div className="py-5">
                <div className="grid grid-cols-2 gap-1">
                <div className="text-center sm:text-left whitespace-nowrap">
                    <button className="transition duration-200 mx-5 px-5 py-4 cursor-pointer font-normal text-sm rounded-lg text-gray-500 hover:bg-gray-200 focus:outline-none focus:bg-gray-300 focus:ring-2 focus:ring-gray-400 focus:ring-opacity-50 ring-inset">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" className="w-4 h-4 inline-block align-text-top">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                        </svg>
                        <Link href="/" className="inline-block ml-1">
                            Back to synocommunity.com
                        </Link>
                    </button>
                </div>
                </div>
            </div>
        </div>
        </div>
    );
}
