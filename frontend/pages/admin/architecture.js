import Layout from "../../components/layout";
import Model from "../../components/model";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { useState } from "react";
import { postJsonForm } from "../../utils";

export default function ArchitecturePage() {
    const url = `http://127.0.0.1:8080/api/architecture`;
    const [data, setData] = useState([]);

    let [isOpen, setIsOpen] = useState(false);
    async function handleSubmit(event) {
        let response = await postJsonForm(url, event, []);
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
            <TablePaginate columns={columns} url={url} data={data} setData={setData}></TablePaginate>
            <Button type="button" onClick={openModal}>
                Add Architecture
            </Button>
            <Model
                isOpen={isOpen}
                setIsOpen={setIsOpen}
                title="Insert Architecture"
                buttons={<Button className="" type="submit">Submit Query</Button>}
            >
                <form onSubmit={handleSubmit}>
                    <label className="block">
                        Architecture:
                        <input name="code" type="text"
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        ></input>
                    </label>
                </form>
            </Model>
        </Layout>
    );
}
