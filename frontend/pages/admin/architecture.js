import Layout from "../../components/layout";
import Model from "../../components/model";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { useState } from "react";

export default function ArchitecturePage({ data }) {
    const url = `http://127.0.0.1:8080/api/architecture`;

    let [isOpen, setIsOpen] = useState(false);
    let [newArchitecture, setNewArchitecture] = useState("");

    function handleChange(event) {
        setNewArchitecture({ value: event.target.value });
    }

    async function handleSubmit(event) {
        event.preventDefault();
        const requestOptions = {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ code: newArchitecture.value }),
        };

        let response = await fetch(
            "http://127.0.0.1:8080/api/architecture",
            requestOptions
        ).catch((err) => console.error(err));
        console.log(await response.json().catch((err) => console.error(err)));
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    const columns = [
        { Header: "ID", accessor: "id" },
        { Header: "Firmware", accessor: "code" },
    ];

    return (
        <Layout>
            <h1>Architecture</h1>
            <TablePaginate columns={columns} url={url}></TablePaginate>
            <Button type="button" onClick={openModal}>
                Add Architecture
            </Button>
            <Model
                isOpen={isOpen}
                setIsOpen={setIsOpen}
                title="Insert Architecture"
            >
                <form
                    onSubmit={handleSubmit}
                    action="http://127.0.0.1:8080/api/architecture"
                    method="post"
                >
                    <label>
                        Architecture:
                        <input
                            value={setNewArchitecture.value}
                            onChange={handleChange}
                            name="architecture"
                            type="text"
                        ></input>
                    </label>
                    <input
                        type="submit"
                        value="Submit Query"
                        className="py-3 px-6 text-white rounded-lg bg-blue-500 shadow-lg block md:inline-block hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-opacity-50"
                    ></input>
                </form>
            </Model>
        </Layout>
    );
}
