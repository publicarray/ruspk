import Layout from "../../components/layout";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import Model from "../../components/model";
import { postJsonForm } from "../../utils";
import DeleteBtn from "../../components/delete-btn";

export default function PackagePage() {
    const url = `http://127.0.0.1:8080/api/package`
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


    let del = async function (index, data) {
        const response = await fetch(`${url}/${data[index].id}`, {
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

    const columns = [
        { Header: 'ID', accessor: 'id'},
        { Header: 'Name', accessor: 'name'},
        { Header: 'Author', accessor: 'author'}, // author_user_id
        // { Header: 'Maintainers', accessor: 'maintainers',},
        { Header: 'Insert Date', accessor: 'insert_date'},
        {
            Header: "Actions",
            accessor: "actions",
            Cell: (props) => {
                return (
                    <div>
                        <span onClick={() => del(props.row.index, props.data)}>
                            <DeleteBtn></DeleteBtn>
                        </span>
                    </div>
                );
            },
        }
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
