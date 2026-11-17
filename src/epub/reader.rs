// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use std::fs::File;
use std::io::Read;
use std::path::Path;

use zip;
use zip::ZipArchive;
use std::fs;
use std::io::BufReader;

use std::collections::HashMap;

use super::Package;


pub struct Reader {

    filename: String,
    opf_path: String,
    root_dir: String,
    package: Package,
    data: HashMap<String, Vec<u8>>,


}

impl Reader {

    fn get_dir_with_slash(s: &str) -> String {
        match s.rfind('/') {
            Some(idx) => s[..=idx].to_string(),
            None => String::new(),
        }
    }

    fn read_file_from_archive
    (
        archive: &mut ZipArchive<File>,
        file_path: &str
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> 
    {

        let mut content_file= archive.by_name(file_path)?;

        let mut content = Vec::<u8>::new();
        content_file.read_to_end(&mut content)?;
        Ok(content)
    }


    pub fn new(filename: &String) -> Result<Self, Box<dyn std::error::Error>> {

        let file = std::fs::File::open(filename)?;
        let mut archive: ZipArchive<File> = ZipArchive::new(file)?;

        let (opf_path, mut package) 
        = super::container::read_opf_from_epub(&mut archive)?;

        println!("Found OPF at: {}", opf_path);
        println!("Title: {:?}", package.metadata.titles.first());

        let root_dir = Self::get_dir_with_slash(&opf_path);

        let mut data = HashMap::<String, Vec<u8>>::new();

        for item in &package.manifest.items {
            println!("ManifestItem: {:?}", item);
            let item_path = format!("{}{}", root_dir, item.href);
            println!("id: {}, href: {}", item.id, item_path);

            let item_content = 
                Self::read_file_from_archive(&mut archive, &item_path)?;
            data.insert(item.id.clone(), item_content);
        }

        Ok(Self {
            filename: filename.clone(),
            opf_path: opf_path,
            root_dir: root_dir,
            package: package,
            data: data
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