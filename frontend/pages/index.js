import Layout from "../components/layout";
import Head from 'next/head'

export default function IndexPage() {
    return (
        <Layout>
            <Head>
                <title>SynoCommunity</title>
            </Head>
            <main className="text-gray-700 dark:text-gray-300 max-w-6xl mx-auto mt-4">
                <div className="text-xl">
                    <h1 className="text-4xl text-gray-900 dark:text-gray-100 mb-4">Packages for Synology NAS</h1>
                    <p className="mb-4">SynoCommunity provides packages for Synology-branded NAS devices. <br></br> Packages are provided for <em>free</em> and made by developers on their free time. See how you can <a className="text-blue-600 hover:text-blue-70 dark:text-blue-400 dark:hover:text-blue-600 0 hover:underline" href="#contribute">contribute</a>.</p>
                    <p className="mb-4">Bandwidth and content delivery is generously provided and sponsored by
                        <a href="https://www.fastly.com/" title="Fastly">
                            <svg xmlns="http://www.w3.org/2000/svg"className="w-16 mx-1 inline hover:scale-110 transition" viewBox="0 0 1709 735" alt="Fastly">
                                <path fill="#FF282D" d="M1135.107 129.93v405.493h121.802v-61.956h-40.25V68.164l-81.564.011zM77.437 473.467h41.405V277.836H77.437v-53.801l41.405-6.809v-54.462c0-65.969 14.359-94.598 98.487-94.598 18.17 0 39.707 2.683 58.563 6.08l-11.176 66.314c-12.77-2.023-19.104-2.382-27.176-2.382-29.639 0-37.133 2.957-37.133 31.916v47.131h61.541v60.611h-61.541v195.632h40.973v61.946l-163.941.017v-61.964zM1093.617 453.889c-12.759 2.692-23.915 2.368-31.988 2.567-33.557.825-30.658-10.204-30.658-41.844V277.836h63.873v-60.611h-63.873V68.164h-81.563v363.358c0 71.341 17.606 103.9 94.339 103.9 18.173 0 43.144-4.676 61.997-8.714l-12.127-72.819zM1600.297 473.8c17.141 0 31.023 13.625 31.023 30.768 0 17.138-13.883 30.76-31.023 30.76-17.141 0-30.941-13.622-30.941-30.76.001-17.143 13.801-30.768 30.941-30.768m0 56.693c14.239 0 25.837-11.688 25.837-25.925 0-14.245-11.598-25.579-25.837-25.579-14.236 0-25.758 11.335-25.758 25.579.001 14.236 11.522 25.925 25.758 25.925m5.713-10.811-6.242-9.141h-4.306v9.141h-6.941v-30.239h12.651c7.473 0 12.133 3.782 12.133 10.468 0 4.914-2.463 8.258-6.327 9.398l7.558 10.372h-8.526zm-10.547-15.207H1601c3.168 0 5.276-1.229 5.276-4.563 0-3.165-2.109-4.402-5.104-4.402h-5.71v8.965zM847.593 277.725v-10.851c-24.675-4.501-49.178-4.566-62.47-4.566-37.957 0-42.585 20.129-42.585 31.04 0 15.436 5.267 23.782 46.388 32.773 60.107 13.494 120.473 27.573 120.473 102.104 0 70.682-36.377 107.198-112.947 107.198-51.238 0-100.954-10.999-138.888-20.625v-60.909h61.764l-.036 10.821c26.579 5.134 54.448 4.62 68.997 4.62 40.487 0 47.037-21.767 47.037-33.34 0-16.061-11.611-23.774-49.562-31.47-71.5-12.217-128.244-36.628-128.244-109.257 0-68.746 45.979-95.714 122.55-95.714 51.875 0 91.318 8.045 129.272 17.676v60.5h-61.749zM472.333 331.263l-6.207-6.209-31.522 27.47a14.995 14.995 0 0 0-5.071-.897c-8.506 0-15.392 7.089-15.392 15.824 0 8.745 6.886 15.832 15.392 15.832 8.502 0 15.404-7.087 15.404-15.832 0-1.659-.252-3.257-.713-4.76l28.109-31.428z"/>
                                <path fill="#FF282D" d="m597.261 453.889-.053-253.81h-81.562v23.802a166.734 166.734 0 0 0-55.455-20.979h.461v-28.166h9.951v-20.714h-82.125v20.714h9.95v28.166h.566c-78.009 14.361-137.126 82.671-137.126 164.833 0 92.595 75.062 167.657 167.657 167.657 31.602 0 61.155-8.755 86.385-23.955l14.694 23.986h86.152v-81.535h-19.495zm-162.851-.185v-9.588h-9.772v9.561c-43.775-2.551-78.789-37.721-81.073-81.567h9.713v-9.772h-9.663c2.596-43.542 37.466-78.378 81.023-80.917v9.61h9.772v-9.638c42.935 2.295 77.52 35.973 81.257 78.51v2.802h-9.791v9.772h9.793l.001 2.676c-3.717 42.557-38.311 76.256-81.26 78.551zM1463.34 217.225h168.223v60.552h-40.209l-103.17 253.82c-29.569 71.3-78.136 138.408-152.102 138.408-18.186 0-42.396-2.015-59.185-6.049l7.377-74.028c10.773 2.015 24.884 3.341 32.288 3.341 34.301 0 72.993-21.253 85.094-58.257L1297.12 277.779h-40.211v-60.552h168.31v60.552h-40.205l59.223 145.702 59.223-145.702h-40.121v-60.554z"/>
                            </svg>
                        </a>
                     .</p>
                    <p className="mb-4">Join us on Discord! <a href="https://discord.gg/nnN9fgE7EF" title="Discord"><img className="inline hover:scale-110 transition" width="194" hight="28" src="https://img.shields.io/discord/732558169863225384?color=7289DA&label=Discord&logo=Discord&logoColor=white&style=for-the-badge" alt="Join Discord Button"/></a></p>
                    <p className="mb-4">Only packages that are modified to be compatible with DSM7 by the associated developers can execute on DSM7.0. If not,the package cannot be able to function after upgrading from DSM6.2 to DSM7.0.</p>
                </div>
                <div id="summary" className="flex text-center md:flex-row flex-col">
                    <div className="m-4">
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-40 w-40 mx-auto" viewBox="0 0 20 20" fill="currentColor">
                            <path fillRule="evenodd" d="M2 9.5A3.5 3.5 0 005.5 13H9v2.586l-1.293-1.293a1 1 0 00-1.414 1.414l3 3a1 1 0 001.414 0l3-3a1 1 0 00-1.414-1.414L11 15.586V13h2.5a4.5 4.5 0 10-.616-8.958 4.002 4.002 0 10-7.753 1.977A3.5 3.5 0 002 9.5zm9 3.5H9V8a1 1 0 012 0v5z" clipRule="evenodd" />
                        </svg>
                        <h2 className="text-3xl my-2">Easy Install</h2>
                        <p>From our server to your NAS in a few clicks. Once our repository is added, you will be able to install packages directly from the Package Center</p>
                        <p><a className="py-2 px-4 my-2 inline-block text-sm text-blue-600 hover:bg-blue-700  dark:text-blue-400 hover:text-white border border-blue-500 hover:border-blue-700 rounded-lg" href="#easy-install" role="button">View details &raquo;</a></p>
                    </div>
                    <div lassName="m-4">
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-40 w-40 mx-auto" viewBox="0 0 20 20" fill="currentColor">
                            <path fillRule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clipRule="evenodd" />
                        </svg>
                        <h2 className="text-3xl my-2">Bleeding Edge</h2>
                        <p>We provide frequent updates to our packages so you can enjoy new features of your favorite softwares. Our packages aim to be compatible with the latest version of Synology DSM upon official release.</p>
                        <p><a className="py-2 px-4 my-2 inline-block text-sm text-blue-600 hover:bg-blue-700  dark:text-blue-400 hover:text-white border border-blue-500 hover:border-blue-700 rounded-lg" href="#bleeding-edge" role="button">View details &raquo;</a></p>
                    </div>
                    <div lassName="m-4">
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-40 w-40 mx-auto" viewBox="0 0 20 20" fill="currentColor">
                            <path fillRule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clipRule="evenodd" />
                        </svg>
                        <h2 className="text-3xl my-2">Open Source</h2>
                        <p>Our cross-compilation framework is open source and allows you to rebuild packages from scratch easily. This ensures maximum security and maintainability.</p>
                        <p><a className="py-2 px-4 my-2 inline-block text-sm text-blue-600 hover:bg-blue-700  dark:text-blue-400 hover:text-white border border-blue-500 hover:border-blue-700 rounded-lg" href="#open-source" role="button">View details &raquo;</a></p>
                    </div>
                </div>
                <div id="easy-install" className="my-10 text-sm">
                    <h2 className="text-2xl my-2">
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-10 w-10 inline mr-2" viewBox="0 0 20 20" fill="currentColor">
                            <path fillRule="evenodd" d="M2 9.5A3.5 3.5 0 005.5 13H9v2.586l-1.293-1.293a1 1 0 00-1.414 1.414l3 3a1 1 0 001.414 0l3-3a1 1 0 00-1.414-1.414L11 15.586V13h2.5a4.5 4.5 0 10-.616-8.958 4.002 4.002 0 10-7.753 1.977A3.5 3.5 0 002 9.5zm9 3.5H9V8a1 1 0 012 0v5z" clipRule="evenodd" />
                        </svg>
                        Easy Install
                    </h2>
                    <h3 className="text-xl my-2">Step 1</h3>
                    <p>Log into your NAS as administrator and go to <strong>Main Menu &rarr; Package Center &rarr; Settings</strong> and set Trust Level to <em>Synology Inc. and trusted publishers</em>.</p>
                    <h3 className="text-xl my-2">Step 2</h3>
                    <p>In the <strong>Package Sources</strong> tab, click <strong>Add</strong>, type <em>SynoCommunity</em> as <strong>Name</strong> and <em><code>https://packages.synocommunity.com/</code></em> as <strong>Location</strong> and then press <strong>OK</strong> to validate.</p>
                    <h3 className="text-xl my-2">Step 3</h3>
                    <p>Go back to the <strong>Package Center</strong> and enjoy SynoCommunity's packages in the <strong>Community</strong> tab.</p>
                </div>
                <div id="bleeding-edge" className="my-10 text-sm">
                    <h2 className="text-2xl my-2">
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-10 w-10 inline mr-2" viewBox="0 0 20 20" fill="currentColor">
                            <path fillRule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clipRule="evenodd" />
                        </svg>
                        Bleeding Edge
                    </h2>
                    <p>Latest stable version of your favorite software</p>
                </div>
                <div id="open-source" className="my-10 text-sm">
                    <h2 className="text-2xl my-2">
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-10 w-10 inline mr-2" viewBox="0 0 20 20" fill="currentColor">
                            <path fillRule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clipRule="evenodd" />
                        </svg>
                        Open Source
                    </h2>
                    <p className="mb-4"><a className="text-blue-600 hover:text-blue-70 dark:text-blue-400 dark:hover:text-blue-600 0 hover:underline" href="https://github.com/SynoCommunity">SynoCommunity</a>'s packages are built with <a className="text-blue-600 hover:text-blue-70 dark:text-blue-400 dark:hover:text-blue-600 0 hover:underline" href="https://github.com/SynoCommunity/spksrc">spksrc</a>, a cross-compilation framework dedicated to build packages for Synology NAS.</p>
                    <h3 id="contribute" className="text-xl my-2">Contribute</h3>
                    <p>We welcome package related bug reports, patches and feature requests on our <a className="text-blue-600 hover:text-blue-70 dark:text-blue-400 dark:hover:text-blue-600 0 hover:underline" href="https://github.com/SynoCommunity/spksrc/issues">issue tracker</a>. You can also support us by donating to our contributors.</p>
                </div>
            </main>
        </Layout>
  )
}
