// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use zip;
use zip::ZipArchive;
use std::fs;
use std::io::BufReader;

use std::collections::HashMap;

use super::Package;


pub struct Reader {

    filename: String,
    //root_path: String,
    //root_dir: String,
    package: Package,
    data: HashMap<String, Vec<u8>>,


}

impl Reader {


    pub fn new(filename: &String) -> Result<Self, &'static str> {


        let file = std::fs::File::open(filename)
            .map_err(|_| "File open failed.")?;
        let mut archive = ZipIterable::try_new(file, zip::read::Config::default())
            .map_err(|_| "Error opening archive.")?;
        /*
        for entry in archive.files().unwrap() {
            let entry = entry.unwrap();
            //println!("{:>20}  {}", entry.size(), entry.name().unwrap());
        }
        */
        let (opf_path, mut package) 
        = super::container::read_opf_from_epub(filename).unwrap();

        println!("Found OPF at: {}", opf_path);
        println!("Title: {:?}", package.metadata.titles.first());

        let mut data = HashMap::<String, Vec<u8>>::new();
        {
            for item in &package.manifest.items {
                println!("ManifestItem: {:?}", item);
            }

        }


        Ok(Self {
            filename: filename.clone(),
            //root_path: String,
            //root_dir: String,
            package: package,
            data: HashMap::<String, Vec<u8>>::new()
    
        })

    }



}
/*
class EPub_Reader:

    def __init__(self, file_path: str):
        self.file_path = file_path
        self.archive = None

    def __enter__(self):
        self.archive = zipfile.ZipFile(self.file_path, 'r')
        self.root_path = self._get_root_path()
        self.root_dir = Path(self.root_path).parent
        self.package: Package = self._read_package()

        # Retrieve the archivew data according to the manifest.
        items = self.package.item_list()
        for item in items:
            id = item["id"]
            href = item["href"]
            epub_path = (self.root_dir.joinpath(href)).as_posix() 
            data  = self.archive.read(epub_path)
            self.package.set_data(id, data) 

        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.archive:
            self.archive.close()

    # Temporary since we can't  generate a opf file yet
    def read_file(self, path):
        return self.archive.read(path)

    def item_list(self):
        return self.package.item_list()

    def get_data(self, id: str):
        return self.package.get_data(id)
    
    def _read_package(self):
        with self.archive.open(self.root_path) as xml_file:
            return Package(tree=etree.parse(xml_file))



    def _get_root_path(self):

        with self.archive.open("META-INF/container.xml") as xml_file:
            # get the value of full-path from  <container><rootfiles><rootfile>
            # if there is more than one <rootfile>, ignore all but the first.
            xml_tree = etree.parse(xml_file)
            xml_root = xml_tree.getroot()
            for rootfile in xml_tree.findall(
            ".//x:rootfile[@media-type]", namespaces={"x": "urn:oasis:names:tc:opendocument:xmlns:container"}
            ):
                if rootfile.get("media-type") == "application/oebps-package+xml":
                    return rootfile.get("full-path")
        raise Epub3_Exception("Couldn't retrieve the rootfile path from META-INF/container.xml.")            
        */