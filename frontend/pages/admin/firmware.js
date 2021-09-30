import Layout from "../../components/layout";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { useState } from "react";
import Model from "../../components/model";
import { postJsonForm } from "../../utils";

export default function FirmwarePage() {
    const url = `http://127.0.0.1:8080/api/firmware`
    let [isOpen, setIsOpen] = useState(false);
    const [data, setData] = useState([]);

    async function handleSubmit(event) {
        let response = await postJsonForm(url, event, []);
        console.log(response);
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    const columns = [
        { Header: 'ID', accessor: 'id',},
        { Header: 'version', accessor: 'version',},
        { Header: 'Build', accessor: 'build',},
    ];

    return (
        <Layout>
            <h1>Firmware</h1>
            <TablePaginate columns={columns} url={url} data={data} setData={setData}></TablePaginate>
            <Button type="button" onClick={openModal}>
                Add Firmware
            </Button>
            <Model
                isOpen={isOpen}
                setIsOpen={setIsOpen}
                title="Insert Firmware"
                buttons={<Button className="" type="submit">Submit Query</Button>}
            >
                <form onSubmit={handleSubmit}>
                    <label className="block">
                        Version:
                        <input name="version" type="text"
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        ></input>
                    </label>
                    <label className="block">
                        Build:
                        <input name="build" type="text"
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        ></input>
                    </label>
                </form>
            </Model>
        </Layout>
    );
}
