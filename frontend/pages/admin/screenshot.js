import Layout from "../../components/layout-admin";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatImage, postJsonForm, API, API_VER } from '../../utils';
import DeleteBtn from "../../components/delete-btn";
import { useRouter } from 'next/router'

export default function ScreenshotPage() {
    const url = `${API}/${API_VER}/screenshot`
    const router = useRouter()
    let [isOpen, setIsOpen] = useState(false);
    const [data, setData] = useState([]);

    let fileInput
    async function handleSubmit(event) {
        let response = await postJsonForm(url, event, []);
        console.log(response);
        setIsOpen(false);
    }

    function openModal() {
        setIsOpen(true);
    }

    // const data = [
    //     { id: 1, package: "Syncthing", image: "https://packages.synocommunity.com/syncthing/screenshot_1.jpg" },
    // ];

    let del = async function (row, data) {
        const response = await fetch(`${url}/${row.values.id}`, {
            headers: {
                'Authorization': 'Bearer ' + localStorage.getItem("jwt")
            },
            method: "DELETE",
        });
        if (response.ok) {
            data.splice(row.index, 1) // update table
            router.push("/admin/screenshot", undefined, {shallow: true}) // force refresh of internal data
        }
    }

    const columns = [
        { Header: 'ID', accessor: 'id' },
        { Header: 'Package', accessor: 'package' },
        { Header: 'Image', accessor: 'path', Cell: ({ value }) => formatImage(value)},
        {
            Header: "Actions",
            accessor: "actions",
            Cell: (props) => {
                return (
                    <div>
                        <span onClick={() => del(props.row, props.data)}>
                            <DeleteBtn></DeleteBtn>
                        </span>
                    </div>
                );
            },
        }
    ];

    return (
        <Layout>
            <h1>Screenshot</h1>
            <TablePaginate columns={columns} url={url} data={data} setData={setData}></TablePaginate>
            <Button type="button" onClick={openModal}>
                Add Screenshot
            </Button>
            <Model
                isOpen={isOpen}
                setIsOpen={setIsOpen}
                title="Insert Architecture"
                buttons={<Button className="" type="submit">Save</Button>}
                onSubmit={handleSubmit}
            >
                <label className="block">
                    Package:
                    <input name="package" type="text"
                        className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    ></input>
                </label>
                {/* <select name="package">
                    <option value="grapefruit">Grapefruit</option>
                    <option value="lime">Lime</option>
                    <option value="coconut">Coconut</option>
                    <option value="mango">Mango</option>
                </select> */}
                <label className="block">
                    Screenshot:
                    <input name="files" type="file" ref={fileInput}
                        className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                    ></input>
                </label>
        </Model>
        </Layout>
    );
}
