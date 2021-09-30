import Layout from "../../components/layout";
import Button from "../../components/button";
import TablePaginate from "../../components/table-paginate";
import Model from "../../components/model";
import { useState, useRef } from "react";
import { Dialog } from "@headlessui/react";
import { formatImage, postJsonForm } from '../../utils';
import DeleteBtn from "../../components/delete-btn";

export default function ScreenshotPage() {
    const url = `http://127.0.0.1:8080/api/screenshot`
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
            >
                <form onSubmit={handleSubmit}>
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
                </form>
            </Model>
        </Layout>
    );
}
