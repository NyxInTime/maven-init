use clap::Parser;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    process::Command,
};

#[derive(Parser, Debug)]
#[command(name = "mvn-init")]
struct Args {
    #[arg(short, long)]
    group: String,

    #[arg(short, long)]
    artifact: String,

    #[arg(short, default_value_t = false)]
    database: bool,
}

fn main() {
    let args = Args::parse();
    let group_id = "-DgroupId=".to_owned() + &args.group;
    let artifact = "-DartifactId=".to_owned() + &args.artifact;
    let mut cmd = Command::new("mvn");

    cmd.args([
        "archetype:generate".to_owned(),
        group_id,
        artifact,
        "-DarchetypeArtifactId=maven-archetype-quickstart".to_owned(),
        "-DarchetypeVersion=1.5".to_owned(),
        "-DinteractiveMode=false".to_owned(),
    ]);
    let _output = cmd.output().unwrap();
    let mut gitignore = File::create(".gitignore").unwrap();
    let group_path: String = args
        .group
        .as_bytes()
        .iter()
        .map(|c| {
            if *c == ".".as_bytes()[0] {
                "/".to_owned()
            } else {
                char::from(*c).to_string()
            }
        })
        .collect();

    if args.database {
        //do stuff to get my db.properties and the right gitignore, otherwise use the basic gitignore
        gitignore
            .write_all(b"target/\n*.class\ndb.properties")
            .unwrap();
        let dbconnection = r#"package {};

import java.io.FileInputStream;
import java.io.IOException;
import java.sql.Connection;
import java.sql.SQLException;
import java.util.Properties;

import javax.sql.DataSource;

import org.postgresql.ds.PGSimpleDataSource;

public class DbConnection {
  private DataSource ds;

  public DbConnection() {
    try {
      ds = createDataSource();
    }
    catch (IOException ioe) {
      throw new RuntimeException(ioe);
    }
  }

  private Properties initProperties() throws IOException {
    try (FileInputStream fis = new FileInputStream("db.properties")) {
      Properties authProps = new Properties();
      authProps.load(fis);
      return authProps;
    }
  }

  private DataSource createDataSource() throws IOException {
    Properties authProps = initProperties();

    PGSimpleDataSource pgds = new PGSimpleDataSource();
    pgds.setUrl(authProps.getProperty("dburl"));
    pgds.setUser(authProps.getProperty("username"));
    pgds.setPassword(authProps.getProperty("password"));

    return pgds;
  }

  public Connection getConnection() throws SQLException {
    return ds.getConnection();
  }
}
"#
        .replace("{}", &args.group);
        let mut properties = File::create("db.properties").unwrap();
        properties.write_all(b"username=<student-id>\npassword=<password>\ndburl=jdbc:postgresql://cspostgres.dawsoncollege.qc.ca:5432/<db-username>?escapeSyntaxCallMode=callIfNoReturn").unwrap();
        let file_path =
            args.artifact.clone() + "/src/main/java/" + &group_path + "/DbConnection.java";
        let mut dbconnectionfile = File::create(file_path).unwrap();
        dbconnectionfile.write_all(dbconnection.as_bytes()).unwrap();
        // edit pom file to add postgres to it
        let psqldep = "<dependency>
    <groupId>org.postgresql</groupId>
    <artifactId>postgresql</artifactId>
    <version>42.7.14</version>
</dependency>";
        let pom_path = args.artifact + "/pom.xml";
        let pom = File::open(&pom_path).unwrap();
        let reader = BufReader::new(&pom);

        let mut new_lines = Vec::new();
        let mut seen = false;
        for line in reader.lines() {
            let line = line.unwrap();
            if line.contains("</dependency>") && !seen {
                new_lines.push(line);
                new_lines.push(psqldep.to_owned());
                seen = true;
            } else {
                new_lines.push(line);
            }
        }
        let to_write: &[u8] = &new_lines.join("\n").into_bytes();
        let _pom_write = fs::write(pom_path, to_write).unwrap();
    } else {
        gitignore.write_all(b"target/\n*.class").unwrap();
    }
}
