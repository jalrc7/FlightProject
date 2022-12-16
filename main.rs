extern crate csv;
use std::collections::HashMap;
use std::collections::LinkedList;

#[allow(dead_code, non_snake_case)]
#[derive(Debug)]
struct Data{
    itinID: Vec<i64>,
    origin: Vec<String>,
    originWac: Vec<i64>,
    dest: Vec<String>,
    destWac: Vec<i64>,
    miles: Vec<i64>,
    pricePerTicket: Vec<i64>,
}

impl Data {
    fn new() -> Data{
        Data{ 
            itinID: Vec::new(), 
            origin: Vec::new(), 
            originWac: Vec::new(), 
            dest: Vec::new(), 
            destWac:Vec::new(), 
            miles: Vec::new(), 
            pricePerTicket: Vec::new(),
        }
    }

    fn read_csv(file: &str, hh: bool) -> Data {
        println!("Using file {}", file);
        let file = std::fs::File::open(file).unwrap();
        let mut rdr = csv::ReaderBuilder:: new()
            .delimiter(b',')
            .has_headers(hh)
            .from_reader(file);
            let mut data_frame = Data::new();
            for result in rdr.records().into_iter(){
                let record= result.unwrap();
                data_frame.push(&record);
            }
            return data_frame;
        }
        fn push(&mut self, row: &csv::StringRecord) {
            self.itinID.push(row[0].parse().unwrap());
            self.origin.push(row[1].to_string());
            self.originWac.push(row[2].parse().unwrap());
            self.dest.push(row[3].to_string());
            self.destWac.push(row[4].parse().unwrap());
            self.miles.push(row[5].parse().unwrap());
            self.pricePerTicket.push(row[6].parse().unwrap());

        }
  }


struct Graph{
    fwd_table: HashMap<i64, LinkedList<i64>>,
    rev_table: HashMap< LinkedList<i64> , i64>, 
    edges: Vec<(i64, i64, i64)>,
    count: i64,
 }

impl Graph {
    fn create(file: &str) -> Graph {
        let mut fh = HashMap::<i64, LinkedList<i64>>::new();
        let mut rh = HashMap::< LinkedList<i64>, i64>::new();
        let mut edges = Vec::<(i64, i64, i64)>::new();
        let data = Data::read_csv("2018_Flights.csv", true);
        let mut count: i64 = 0;
        let mut x: i64;
        let mut y: i64;
        let mut z: i64;

        (0..data.originWac.len()).for_each(|i| {
            //assumes all entries are the same length
            let originWac= &data.originWac[i];
            let origin = &data.origin[i];
            let dest = &data.dest[i];
            let destWac = &data.destWac[i];
            let miles = &data.miles[i];
            let priceperticket = &data.pricePerTicket[i];
            let mut weight = priceperticket/miles;
        
            let entry = fh.get(originWac);
            match entry{
                None => {
                    let mut list: LinkedList<&i64> = LinkedList::new();
                    list.push_front(destWac);
                    list.push_front(miles);
                    list.push_front(priceperticket);
                    fh.insert(originWac, list);
                    rh.insert(list, originWac);
                    
                }
                Some(originWac) => x = originWac,
            }
            let entry = fh.get(destWac);
            match entry{
                None => {
                    let mut list: LinkedList<&i64> = LinkedList::new();
                    list.push_front(originWac);
                    list.push_front(miles);
                    list.push_front(priceperticket);
                    fh.insert(destWac, list);
                    rh.insert(list, destWac);
                    
                }
                Some(destWac) => y = destWac,
            }
            let entry = fh.get(weight);
            match entry{
                None => {
                    let mut list: LinkedList<&i64> = LinkedList::new();
                    list.push_front(originWac);
                    list.push_front(miles);
                    list.push_front(priceperticket);
                    fh.insert(destWac, list);
                    rh.insert(list, destWac);
                    
                }
                Some(weight) => z = weight,
            }
            edges.push((x,y,z));
            count+= 1;
        });

        return Graph { fwd_table: fh, rev_table: rh, edges, count};

    }

    fn create_undirected(n: isize, outedges: isize) -> Graph {
        let parent: Vec<i64> = vec![];
        let rank: Vec<i64> = vec![];
        let mut graph = Graph::create("2018_Flight.csv");
        let n = graph.count();
        g
    }

    fn find(parent: Vec<i64> , x: i64) -> i64 {
        let i: isize = x;
        if parent[i] != x {
            parent[i] = find(parent, parent[i]);
        }
        return parent[i];
    }

    fn KruskalMST(&mut Graph) -> Vec<Edge> {
        let mut result: Vec<Edge> = vec![];
        for node in 0..self.count {
            self.parent.push(node);
            self.rank.push(0);
        }
        let mut num_mst_e = 0;
        let mut next_edge = 0;
        while num_mst_e < self.n - 1 {
            let (x,y,z) = self.outedges[next_edge];
            next_edge = next_edge + 1;
            let x = graph.find(u);
            let y = graph.find(v);
            let x =
            if x != y {
                num_mst_e += 1;
                result.push((u,v,w));
                self.union(x,y);
            }
        }
        result
        }
    }

fn main()  {
    let mut graph = Graph::create("2018_Flight.csv");
    io::stdout().flush().unwrap();

}
