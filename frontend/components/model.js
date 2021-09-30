import { Dialog, Transition } from "@headlessui/react";
import { Fragment, useState } from "react";
import Button from "./button";

// https://headlessui.dev/react/dialog
export default function Model({ isOpen, setIsOpen, title, description, children, buttons, close = "Cancel" }) {
    function closeModal() {
        setIsOpen(false);
    }

    return (
        <>
            <Transition appear show={isOpen} as={Fragment}>
                <Dialog
                    as="div"
                    className="fixed inset-0 z-10 overflow-y-auto"
                    onClose={closeModal}
                >
                    <div className="min-h-screen p-10">
                        <Transition.Child
                            as={Fragment}
                            enter="ease-out duration-300"
                            enterFrom="opacity-0"
                            enterTo="opacity-100"
                            leave="ease-in duration-200"
                            leaveFrom="opacity-100"
                            leaveTo="opacity-0"
                        >
                            <Dialog.Overlay className="fixed inset-0 bg-black bg-opacity-80" />
                        </Transition.Child>

                        {/* This element is to trick the browser into centering the modal contents. */}
                        {/* <span
                            className="inline-block h-screen align-middle"
                            aria-hidden="true"
                        >
                            &#8203;
                        </span> */}
                        <Transition.Child
                            as={Fragment}
                            enter="ease-out duration-300"
                            enterFrom="opacity-0 scale-95"
                            enterTo="opacity-100 scale-100"
                            leave="ease-in duration-200"
                            leaveFrom="opacity-100 scale-100"
                            leaveTo="opacity-0 scale-95"
                        >
                            <div className="flex flex-col h-full justify-between max-w-lg m-auto transform bg-white shadow-m rounded-3xl">
                                <header className="p-6 mb-4 text-xl font-medium leading-6 text-white bg-gray-700 rounded-t-2xl">
                                    <Dialog.Title as="h3">
                                        { title }
                                    </Dialog.Title>
                                </header>
                                <main className="px-6">
                                    {description &&
                                        <Dialog.Description className="mb-2 text-gray-700">
                                            { description }
                                        </Dialog.Description>
                                    }
                                    {children &&
                                        <div className="mb-2 text-gray-700">
                                            { children }
                                        </div>
                                    }
                                </main>
                                <footer className="p-6 mt-4 bg-gray-100 flex justify-end rounded-b-2xl">
                                    <Button onClick={closeModal} className="mr-4 bg-gray-500 hover:bg-gray-700">{close}</Button>
                                    { buttons }
                                </footer>
                            </div>
                        </Transition.Child>
                    </div>
                </Dialog>
            </Transition>
        </>
    );
}
