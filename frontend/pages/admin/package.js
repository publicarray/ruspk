import Layout from "../../components/layout-admin";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import Model from "../../components/model";
import { postJsonForm, API, API_VER } from "../../utils";
import DeleteBtn from "../../components/delete-btn";
import { createColumnHelper } from "@tanstack/react-table";

export const config = {
    runtime: 'experimental-edge',
}

export default function PackagePage() {
    const url = `${API}/${API_VER}/package`
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


    let del = async function (id, index) {
        const response = await fetch(`${url}/${id}`, {
            headers: {
                'Authorization': 'Bearer ' + localStorage.getItem("jwt")
            },
            method: "DELETE",
        });

        if (response.ok) {
            let data_copy = [...data];
            data_copy.splice(index, 1)
            setData(data_copy);
        }
    }

    // const data = [
    //     { id: 1, name: "bitlbee", author: "Diaoul", maintainers: "", insert_date: "2015-01-28 22:00:44.967691" },
    // ];


    const columnHelper = createColumnHelper();
    const columns = [
        columnHelper.accessor("id"),
        columnHelper.accessor("name"),
        columnHelper.accessor("author"),
        // columnHelper.accessor("maintainers"),
        columnHelper.accessor("insert_date", {
            header: "Insert Data",
        }),
        columnHelper.accessor("actions", {
            header: "Actions",
            cell:  (info) => {
            return (
                <div>
                    <span onClick={i => del(info.row.original.id, info.row.index)}>
                        <DeleteBtn></DeleteBtn>
                    </span>
                </div>
            );
            },
        }),
    ];

    return (
        <Layout>
            <h1>Package</h1>
            <TablePaginate columns={columns} url={url} data={data} setData={setData}></TablePaginate>
            <Button type="button" onClick={openModal}>Add Package</Button>
            <Model
                isOpen={isOpen}
                setIsOpen={setIsOpen}
                title="Insert Package"
                buttons={<Button className="" type="submit">Submit Query</Button>}
            >
                <form onSubmit={handleSubmit}>
                    <label className="block">
                        Name:
                        <input name="name" type="text"
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        ></input>
                    </label>
                    <label className="block">
                        Author:
                        <input name="author" type="text"
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        ></input>
                    </label>
                    <label className="block">
                        Maintainers:
                        <select multiple name="package"
                            className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                        >
                            <option value="grapefruit">Grapefruit</option>
                            <option value="lime">Lime</option>
                            <option value="coconut">Coconut</option>
                            <option value="mango">Mango</option>
                        </select>
                    </label>
                </form>
            </Model>
        </Layout>
    );
}
