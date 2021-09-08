use super::node::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::string::ToString;

// Leecode上的一个最优解
impl ToString for TreeNode {
    fn to_string(&self) -> String {
        format!("{}", self.val)
    }
}

struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut vec_deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

        let mut result_string = String::from("");

        vec_deque.push_back(root);
        let mut current_level = 0;
        let mut next_level_has_some = true;
        while !vec_deque.is_empty() && next_level_has_some {
            let mut level_size = vec_deque.len();

            next_level_has_some = false;
            while level_size > 0 {
                level_size -= 1;
                if current_level != 0 {
                    result_string.push_str(",");
                }

                let r = vec_deque.pop_back().unwrap();
                match r {
                    Some(x) => {
                        let x = x.borrow();
                        result_string.push_str(&x.to_string());
                        vec_deque.push_front(x.left.clone());
                        vec_deque.push_front(x.right.clone());

                        if x.left.is_some() || x.right.is_some() {
                            next_level_has_some = true;
                        }
                    }
                    None => {
                        result_string.push_str("n");
                    }
                }
            }
            current_level += 1;
        }

        return format!("[{}]", result_string);
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        // remove square brackets
        let data: &str = &data[1..data.len() - 1];

        let mut values: VecDeque<Option<i32>> =
            data.split(",").map(|x| x.parse::<i32>().ok()).collect();

        let mut need_children: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let root_val = values.pop_front().unwrap();

        let mut root_node: Option<Rc<RefCell<TreeNode>>> = None;

        if let Some(rv) = root_val {
            let node = TreeNode::new(rv);
            let node = Rc::new(RefCell::new(node));
            root_node = Some(Rc::clone(&node));
            need_children.push_front(Some(Rc::clone(&node)));
        }

        while !need_children.is_empty() && !values.is_empty() {
            let node = need_children.pop_front().unwrap().unwrap();

            let value = values.pop_front().unwrap();
            // add children to node
            if let Some(v) = value {
                let child = TreeNode::new(v);
                let child = Rc::new(RefCell::new(child));
                need_children.push_back(Some(Rc::clone(&child)));
                node.borrow_mut().left = Some(Rc::clone(&child));
            }

            let value = values.pop_front().unwrap();
            // add children to node
            if let Some(v) = value {
                let child = TreeNode::new(v);
                let child = Rc::new(RefCell::new(child));
                need_children.push_back(Some(Rc::clone(&child)));
                node.borrow_mut().right = Some(Rc::clone(&child));
            }
        }

        return root_node;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::TreeNode;
    use assert::Judge;

    #[test]
    fn test_codec2() {
        /**
        serialize = [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9,null,10,null,11,null,12,null,13,null,14,null,15,null,16,null,17,null,18,null,19,null,20,null,21,null,22,null,23,null,24,null,25,null,26,null,27,null,28,null,29,null,30,null,31,null,32,null,33,null,34,null,35,null,36,null,37,null,38,null,39,null,40,null,41,null,42,null,43,null,44,null,45,null,46,null,47,null,48,null,49,null,50,null,51,null,52,null,53,null,54,null,55,null,56,null,57,null,58,null,59,null,60,null,61,null,62,null,63,null,64,null,65,null,66,null,67,null,68,null,69,null,70,null,71,null,72,null,73,null,74,null,75,null,76,null,77,null,78,null,79,null,80,null,81,null,82,null,83,null,84,null,85,null,86,null,87,null,88,null,89,null,90,null,91,null,92,null,93,null,94,null,95,null,96,null,97,null,98,null,99,null,100,null,101,null,102,null,103,null,104,null,105,null,106,null,107,null,108,null,109,null,110,null,111,null,112,null,113,null,114,null,115,null,116,null,117,null,118,null,119,null,120,null,121,null,122,null,123,null,124,null,125,null,126,null,127,null,128,null,129,null,130,null,131,null,132,null,133,null,134,null,135,null,136,null,137,null,138,null,139,null,140,null,141,null,142,null,143,null,144,null,145,null,146,null,147,null,148,null,149,null,150,null,151,null,152,null,153,null,154,null,155,null,156,null,157,null,158,null,159,null,160,null,161,null,162,null,163,null,164,null,165,null,166,null,167,null,168,null,169,null,170,null,171,null,172,null,173,null,174,null,175,null,176,null,177,null,178,null,179,null,180,null,181,null,182,null,183,null,184,null,185,null,186,null,187,null,188,null,189,null,190,null,191,null,192,null,193,null,194,null,195,null,196,null,197,null,198,null,199,null,200,null,201,null,202,null,203,null,204,null,205,null,206,null,207,null,208,null,209,null,210,null,211,null,212,null,213,null,214,null,215,null,216,null,217,null,218,null,219,null,220,null,221,null,222,null,223,null,224,null,225,null,226,null,227,null,228,null,229,null,230,null,231,null,232,null,233,null,234,null,235,null,236,null,237,null,238,null,239,null,240,null,241,null,242,null,243,null,244,null,245,null,246,null,247,null,248,null,249,null,250,null,251,null,252,null,253,null,254,null,255,null,256,null,257,null,258,null,259,null,260,null,261,null,262,null,263,null,264,null,265,null,266,null,267,null,268,null,269,null,270,null,271,null,272,null,273,null,274,null,275,null,276,null,277,null,278,null,279,null,280,null,281,null,282,null,283,null,284,null,285,null,286,null,287,null,288,null,289,null,290,null,291,null,292,null,293,null,294,null,295,null,296,null,297,null,298,null,299,null,300,null,301,null,302,null,303,null,304,null,305,null,306,null,307,null,308,null,309,null,310,null,311,null,312,null,313,null,314,null,315,null,316,null,317,null,318,null,319,null,320,null,321,null,322,null,323,null,324,null,325,null,326,null,327,null,328,null,329,null,330,null,331,null,332,null,333,null,334,null,335,null,336,null,337,null,338,null,339,null,340,null,341,null,342,null,343,null,344,null,345,null,346,null,347,null,348,null,349,null,350,null,351,null,352,null,353,null,354,null,355,null,356,null,357,null,358,null,359,null,360,null,361,null,362,null,363,null,364,null,365,null,366,null,367,null,368,null,369,null,370,null,371,null,372,null,373,null,374,null,375,null,376,null,377,null,378,null,379,null,380,null,381,null,382,null,383,null,384,null,385,null,386,null,387,null,388,null,389,null,390,null,391,null,392,null,393,null,394,null,395,null,396,null,397,null,398,null,399,null,400,null,401,null,402,null,403,null,404,null,405,null,406,null,407,null,408,null,409,null,410,null,411,null,412,null,413,null,414,null,415,null,416,null,417,null,418,null,419,null,420,null,421,null,422,null,423,null,424,null,425,null,426,null,427,null,428,null,429,null,430,null,431,null,432,null,433,null,434,null,435,null,436,null,437,null,438,null,439,null,440,null,441,null,442,null,443,null,444,null,445,null,446,null,447,null,448,null,449,null,450,null,451,null,452,null,453,null,454,null,455,null,456,null,457,null,458,null,459,null,460,null,461,null,462,null,463,null,464,null,465,null,466,null,467,null,468,null,469,null,470,null,471,null,472,null,473,null,474,null,475,null,476,null,477,null,478,null,479,null,480,null,481,null,482,null,483,null,484,null,485,null,486,null,487,null,488,null,489,null,490,null,491,null,492,null,493,null,494,null,495,null,496,null,497,null,498,null,499,null,500,null,501,null,502,null,503,null,504,null,505,null,506,null,507,null,508,null,509,null,510,null,511,null,512,null,513,null,514,null,515,null,516,null,517,null,518,null,519,null,520,null,521,null,522,null,523,null,524,null,525,null,526,null,527,null,528,null,529,null,530,null,531,null,532,null,533,null,534,null,535,null,536,null,537,null,538,null,539,null,540,null,541,null,542,null,543,null,544,null,545,null,546,null,547,null,548,null,549,null,550,null,551,null,552,null,553,null,554,null,555,null,556,null,557,null,558,null,559,null,560,null,561,null,562,null,563,null,564,null,565,null,566,null,567,null,568,null,569,null,570,null,571,null,572,null,573,null,574,null,575,null,576,null,577,null,578,null,579,null,580,null,581,null,582,null,583,null,584,null,585,null,586,null,587,null,588,null,589,null,590,null,591,null,592,null,593,null,594,null,595,null,596,null,597,null,598,null,599,null,600,null,601,null,602,null,603,null,604,null,605,null,606,null,607,null,608,null,609,null,610,null,611,null,612,null,613,null,614,null,615,null,616,null,617,null,618,null,619,null,620,null,621,null,622,null,623,null,624,null,625,null,626,null,627,null,628,null,629,null,630,null,631,null,632,null,633,null,634,null,635,null,636,null,637,null,638,null,639,null,640,null,641,null,642,null,643,null,644,null,645,null,646,null,647,null,648,null,649,null,650,null,651,null,652,null,653,null,654,null,655,null,656,null,657,null,658,null,659,null,660,null,661,null,662,null,663,null,664,null,665,null,666,null,667,null,668,null,669,null,670,null,671,null,672,null,673,null,674,null,675,null,676,null,677,null,678,null,679,null,680,null,681,null,682,null,683,null,684,null,685,null,686,null,687,null,688,null,689,null,690,null,691,null,692,null,693,null,694,null,695,null,696,null,697,null,698,null,699,null,700,null,701,null,702,null,703,null,704,null,705,null,706,null,707,null,708,null,709,null,710,null,711,null,712,null,713,null,714,null,715,null,716,null,717,null,718,null,719,null,720,null,721,null,722,null,723,null,724,null,725,null,726,null,727,null,728,null,729,null,730,null,731,null,732,null,733,null,734,null,735,null,736,null,737,null,738,null,739,null,740,null,741,null,742,null,743,null,744,null,745,null,746,null,747,null,748,null,749,null,750,null,751,null,752,null,753,null,754,null,755,null,756,null,757,null,758,null,759,null,760,null,761,null,762,null,763,null,764,null,765,null,766,null,767,null,768,null,769,null,770,null,771,null,772,null,773,null,774,null,775,null,776,null,777,null,778,null,779,null,780,null,781,null,782,null,783,null,784,null,785,null,786,null,787,null,788,null,789,null,790,null,791,null,792,null,793,null,794,null,795,null,796,null,797,null,798,null,799,null,800,null,801,null,802,null,803,null,804,null,805,null,806,null,807,null,808,null,809,null,810,null,811,null,812,null,813,null,814,null,815,null,816,null,817,null,818,null,819,null,820,null,821,null,822,null,823,null,824,null,825,null,826,null,827,null,828,null,829,null,830,null,831,null,832,null,833,null,834,null,835,null,836,null,837,null,838,null,839,null,840,null,841,null,842,null,843,null,844,null,845,null,846,null,847,null,848,null,849,null,850,null,851,null,852,null,853,null,854,null,855,null,856,null,857,null,858,null,859,null,860,null,861,null,862,null,863,null,864,null,865,null,866,null,867,null,868,null,869,null,870,null,871,null,872,null,873,null,874,null,875,null,876,null,877,null,878,null,879,null,880,null,881,null,882,null,883,null,884,null,885,null,886,null,887,null,888,null,889,null,890,null,891,null,892,null,893,null,894,null,895,null,896,null,897,null,898,null,899,null,900,null,901,null,902,null,903,null,904,null,905,null,906,null,907,null,908,null,909,null,910,null,911,null,912,null,913,null,914,null,915,null,916,null,917,null,918,null,919,null,920,null,921,null,922,null,923,null,924,null,925,null,926,null,927,null,928,null,929,null,930,null,931,null,932,null,933,null,934,null,935,null,936,null,937,null,938,null,939,null,940,null,941,null,942,null,943,null,944,null,945,null,946,null,947,null,948,null,949,null,950,null,951,null,952,null,953,null,954,null,955,null,956,null,957,null,958,null,959,null,960,null,961,null,962,null,963,null,964,null,965,null,966,null,967,null,968,null,969,null,970,null,971,null,972,null,973,null,974,null,975,null,976,null,977,null,978,null,979,null,980,null,981,null,982,null,983,null,984,null,985,null,986,null,987,null,988,null,989,null,990,null,991,null,992,null,993,null,994,null,995,null,996,null,997,null,998,null,999]
        [1,n,2,n,3,n,4,n,5,n,6,n,7,n,8,n,9,n,10,n,11,n,12,n,13,n,14,n,15,n,16,n,17,n,18,n,19,n,20,n,21,n,22,n,23,n,24,n,25,n,26,n,27,n,28,n,29,n,30,n,31,n,32,n,33,n,34,n,35,n,36,n,37,n,38,n,39,n,40,n,41,n,42,n,43,n,44,n,45,n,46,n,47,n,48,n,49,n,50,n,51,n,52,n,53,n,54,n,55,n,56,n,57,n,58,n,59,n,60,n,61,n,62,n,63,n,64,n,65,n,66,n,67,n,68,n,69,n,70,n,71,n,72,n,73,n,74,n,75,n,76,n,77,n,78,n,79,n,80,n,81,n,82,n,83,n,84,n,85,n,86,n,87,n,88,n,89,n,90,n,91,n,92,n,93,n,94,n,95,n,96,n,97,n,98,n,99,n,100,n,101,n,102,n,103,n,104,n,105,n,106,n,107,n,108,n,109,n,110,n,111,n,112,n,113,n,114,n,115,n,116,n,117,n,118,n,119,n,120,n,121,n,122,n,123,n,124,n,125,n,126,n,127,n,128,n,129,n,130,n,131,n,132,n,133,n,134,n,135,n,136,n,137,n,138,n,139,n,140,n,141,n,142,n,143,n,144,n,145,n,146,n,147,n,148,n,149,n,150,n,151,n,152,n,153,n,154,n,155,n,156,n,157,n,158,n,159,n,160,n,161,n,162,n,163,n,164,n,165,n,166,n,167,n,168,n,169,n,170,n,171,n,172,n,173,n,174,n,175,n,176,n,177,n,178,n,179,n,180,n,181,n,182,n,183,n,184,n,185,n,186,n,187,n,188,n,189,n,190,n,191,n,192,n,193,n,194,n,195,n,196,n,197,n,198,n,199,n,200,n,201,n,202,n,203,n,204,n,205,n,206,n,207,n,208,n,209,n,210,n,211,n,212,n,213,n,214,n,215,n,216,n,217,n,218,n,219,n,220,n,221,n,222,n,223,n,224,n,225,n,226,n,227,n,228,n,229,n,230,n,231,n,232,n,233,n,234,n,235,n,236,n,237,n,238,n,239,n,240,n,241,n,242,n,243,n,244,n,245,n,246,n,247,n,248,n,249,n,250,n,251,n,252,n,253,n,254,n,255,n,256,n,257,n,258,n,259,n,260,n,261,n,262,n,263,n,264,n,265,n,266,n,267,n,268,n,269,n,270,n,271,n,272,n,273,n,274,n,275,n,276,n,277,n,278,n,279,n,280,n,281,n,282,n,283,n,284,n,285,n,286,n,287,n,288,n,289,n,290,n,291,n,292,n,293,n,294,n,295,n,296,n,297,n,298,n,299,n,300,n,301,n,302,n,303,n,304,n,305,n,306,n,307,n,308,n,309,n,310,n,311,n,312,n,313,n,314,n,315,n,316,n,317,n,318,n,319,n,320,n,321,n,322,n,323,n,324,n,325,n,326,n,327,n,328,n,329,n,330,n,331,n,332,n,333,n,334,n,335,n,336,n,337,n,338,n,339,n,340,n,341,n,342,n,343,n,344,n,345,n,346,n,347,n,348,n,349,n,350,n,351,n,352,n,353,n,354,n,355,n,356,n,357,n,358,n,359,n,360,n,361,n,362,n,363,n,364,n,365,n,366,n,367,n,368,n,369,n,370,n,371,n,372,n,373,n,374,n,375,n,376,n,377,n,378,n,379,n,380,n,381,n,382,n,383,n,384,n,385,n,386,n,387,n,388,n,389,n,390,n,391,n,392,n,393,n,394,n,395,n,396,n,397,n,398,n,399,n,400,n,401,n,402,n,403,n,404,n,405,n,406,n,407,n,408,n,409,n,410,n,411,n,412,n,413,n,414,n,415,n,416,n,417,n,418,n,419,n,420,n,421,n,422,n,423,n,424,n,425,n,426,n,427,n,428,n,429,n,430,n,431,n,432,n,433,n,434,n,435,n,436,n,437,n,438,n,439,n,440,n,441,n,442,n,443,n,444,n,445,n,446,n,447,n,448,n,449,n,450,n,451,n,452,n,453,n,454,n,455,n,456,n,457,n,458,n,459,n,460,n,461,n,462,n,463,n,464,n,465,n,466,n,467,n,468,n,469,n,470,n,471,n,472,n,473,n,474,n,475,n,476,n,477,n,478,n,479,n,480,n,481,n,482,n,483,n,484,n,485,n,486,n,487,n,488,n,489,n,490,n,491,n,492,n,493,n,494,n,495,n,496,n,497,n,498,n,499,n,500,n,501,n,502,n,503,n,504,n,505,n,506,n,507,n,508,n,509,n,510,n,511,n,512,n,513,n,514,n,515,n,516,n,517,n,518,n,519,n,520,n,521,n,522,n,523,n,524,n,525,n,526,n,527,n,528,n,529,n,530,n,531,n,532,n,533,n,534,n,535,n,536,n,537,n,538,n,539,n,540,n,541,n,542,n,543,n,544,n,545,n,546,n,547,n,548,n,549,n,550,n,551,n,552,n,553,n,554,n,555,n,556,n,557,n,558,n,559,n,560,n,561,n,562,n,563,n,564,n,565,n,566,n,567,n,568,n,569,n,570,n,571,n,572,n,573,n,574,n,575,n,576,n,577,n,578,n,579,n,580,n,581,n,582,n,583,n,584,n,585,n,586,n,587,n,588,n,589,n,590,n,591,n,592,n,593,n,594,n,595,n,596,n,597,n,598,n,599,n,600,n,601,n,602,n,603,n,604,n,605,n,606,n,607,n,608,n,609,n,610,n,611,n,612,n,613,n,614,n,615,n,616,n,617,n,618,n,619,n,620,n,621,n,622,n,623,n,624,n,625,n,626,n,627,n,628,n,629,n,630,n,631,n,632,n,633,n,634,n,635,n,636,n,637,n,638,n,639,n,640,n,641,n,642,n,643,n,644,n,645,n,646,n,647,n,648,n,649,n,650,n,651,n,652,n,653,n,654,n,655,n,656,n,657,n,658,n,659,n,660,n,661,n,662,n,663,n,664,n,665,n,666,n,667,n,668,n,669,n,670,n,671,n,672,n,673,n,674,n,675,n,676,n,677,n,678,n,679,n,680,n,681,n,682,n,683,n,684,n,685,n,686,n,687,n,688,n,689,n,690,n,691,n,692,n,693,n,694,n,695,n,696,n,697,n,698,n,699,n,700,n,701,n,702,n,703,n,704,n,705,n,706,n,707,n,708,n,709,n,710,n,711,n,712,n,713,n,714,n,715,n,716,n,717,n,718,n,719,n,720,n,721,n,722,n,723,n,724,n,725,n,726,n,727,n,728,n,729,n,730,n,731,n,732,n,733,n,734,n,735,n,736,n,737,n,738,n,739,n,740,n,741,n,742,n,743,n,744,n,745,n,746,n,747,n,748,n,749,n,750,n,751,n,752,n,753,n,754,n,755,n,756,n,757,n,758,n,759,n,760,n,761,n,762,n,763,n,764,n,765,n,766,n,767,n,768,n,769,n,770,n,771,n,772,n,773,n,774,n,775,n,776,n,777,n,778,n,779,n,780,n,781,n,782,n,783,n,784,n,785,n,786,n,787,n,788,n,789,n,790,n,791,n,792,n,793,n,794,n,795,n,796,n,797,n,798,n,799,n,800,n,801,n,802,n,803,n,804,n,805,n,806,n,807,n,808,n,809,n,810,n,811,n,812,n,813,n,814,n,815,n,816,n,817,n,818,n,819,n,820,n,821,n,822,n,823,n,824,n,825,n,826,n,827,n,828,n,829,n,830,n,831,n,832,n,833,n,834,n,835,n,836,n,837,n,838,n,839,n,840,n,841,n,842,n,843,n,844,n,845,n,846,n,847,n,848,n,849,n,850,n,851,n,852,n,853,n,854,n,855,n,856,n,857,n,858,n,859,n,860,n,861,n,862,n,863,n,864,n,865,n,866,n,867,n,868,n,869,n,870,n,871,n,872,n,873,n,874,n,875,n,876,n,877,n,878,n,879,n,880,n,881,n,882,n,883,n,884,n,885,n,886,n,887,n,888,n,889,n,890,n,891,n,892,n,893,n,894,n,895,n,896,n,897,n,898,n,899,n,900,n,901,n,902,n,903,n,904,n,905,n,906,n,907,n,908,n,909,n,910,n,911,n,912,n,913,n,914,n,915,n,916,n,917,n,918,n,919,n,920,n,921,n,922,n,923,n,924,n,925,n,926,n,927,n,928,n,929,n,930,n,931,n,932,n,933,n,934,n,935,n,936,n,937,n,938,n,939,n,940,n,941,n,942,n,943,n,944,n,945,n,946,n,947,n,948,n,949,n,950,n,951,n,952,n,953,n,954,n,955,n,956,n,957,n,958,n,959,n,960,n,961,n,962,n,963,n,964,n,965,n,966,n,967,n,968,n,969,n,970,n,971,n,972,n,973,n,974,n,975,n,976,n,977,n,978,n,979,n,980,n,981,n,982,n,983,n,984,n,985,n,986,n,987,n,988,n,989,n,990,n,991,n,992,n,993,n,994,n,995,n,996,n,997,n,998,n,999]
        */
        let mut preorder_long = Vec::<i32>::new();
        let mut inorder_long = Vec::<i32>::new();
        for i in 1..1000 {
            preorder_long.push(i);
            inorder_long.push(i);
        }

        let test_example = vec![
            (vec![1, 2, 3, 4, 5], vec![2, 1, 4, 3, 5]),
            (vec![3, 9, 8, 5, 20, 15, 7], vec![8, 9, 5, 3, 15, 20, 7]),
            (preorder_long, inorder_long),
        ];

        for (preorder, inorder) in test_example {
            let root = TreeNode::new_with_pre_and_in(&preorder, &inorder);
            let codec = Codec::new();
            let serialize = codec.serialize(root);
            println!("serialize = {}", serialize);

            let root_new = codec.deserialize(serialize);
            println!("root_new = {:?}", root_new);
        }
    }
}
