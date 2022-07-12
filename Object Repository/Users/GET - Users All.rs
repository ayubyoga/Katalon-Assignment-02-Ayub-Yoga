<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET - Users All</name>
   <tag></tag>
   <elementGuidId>8bdef5c6-da12-4a25-851e-f4ad384b5d23</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)
WS.verifyElementPropertyValue(response, '[0].id', '1')
WS.verifyElementPropertyValue(response, '[0].name', 'Leanne Graham')
WS.verifyElementPropertyValue(response, '[0].username', 'Bret')
WS.verifyElementPropertyValue(response, '[0].email', 'Sincere@april.biz')
WS.verifyElementPropertyValue(response, '[0].address.street', 'Kulas Light')
WS.verifyElementPropertyValue(response, '[0].address.suite', 'Apt. 556')
WS.verifyElementPropertyValue(response, '[0].address.city', 'Gwenborough')
WS.verifyElementPropertyValue(response, '[0].address.zipcode', '92998-3874')
WS.verifyElementPropertyValue(response, '[0].address.geo.lat', '-37.3159')
WS.verifyElementPropertyValue(response, '[0].address.geo.lng', '81.1496')
WS.verifyElementPropertyValue(response, '[0].phone', '1-770-736-8031 x56442')
WS.verifyElementPropertyValue(response, '[0].website', 'hildegard.org')
WS.verifyElementPropertyValue(response, '[0].company.name', 'Romaguera-Crona')
WS.verifyElementPropertyValue(response, '[0].company.catchPhrase', 'Multi-layered client-server neural-net')
WS.verifyElementPropertyValue(response, '[0].company.bs', 'harness real-time e-markets')


WS.verifyElementPropertyValue(response, '[1].id', '2')
WS.verifyElementPropertyValue(response, '[1].name', 'Ervin Howell')
WS.verifyElementPropertyValue(response, '[1].username', 'Antonette')
WS.verifyElementPropertyValue(response, '[1].email', 'Shanna@melissa.tv')
WS.verifyElementPropertyValue(response, '[1].address.street', 'Victor Plains')
WS.verifyElementPropertyValue(response, '[1].address.suite', 'Suite 879')
WS.verifyElementPropertyValue(response, '[1].address.city', 'Wisokyburgh')
WS.verifyElementPropertyValue(response, '[1].address.zipcode', '90566-7771')
WS.verifyElementPropertyValue(response, '[1].address.geo.lat', '-43.9509')
WS.verifyElementPropertyValue(response, '[1].address.geo.lng', '-34.4618')
WS.verifyElementPropertyValue(response, '[1].phone', '010-692-6593 x09125')
WS.verifyElementPropertyValue(response, '[1].website', 'anastasia.net')
WS.verifyElementPropertyValue(response, '[1].company.name', 'Deckow-Crist')
WS.verifyElementPropertyValue(response, '[1].company.catchPhrase', 'Proactive didactic contingency')
WS.verifyElementPropertyValue(response, '[1].company.bs', 'synergize scalable supply-chains')


WS.verifyElementPropertyValue(response, '[2].id', '3')
WS.verifyElementPropertyValue(response, '[2].name', 'Clementine Bauch')
WS.verifyElementPropertyValue(response, '[2].username', 'Samantha')
WS.verifyElementPropertyValue(response, '[2].email', 'Nathan@yesenia.net')
WS.verifyElementPropertyValue(response, '[2].address.street', 'Douglas Extension')
WS.verifyElementPropertyValue(response, '[2].address.suite', 'Suite 847')
WS.verifyElementPropertyValue(response, '[2].address.city', 'McKenziehaven')
WS.verifyElementPropertyValue(response, '[2].address.zipcode', '59590-4157')
WS.verifyElementPropertyValue(response, '[2].address.geo.lat', '-68.6102')
WS.verifyElementPropertyValue(response, '[2].address.geo.lng', '-47.0653')
WS.verifyElementPropertyValue(response, '[2].phone', '1-463-123-4447')
WS.verifyElementPropertyValue(response, '[2].website', 'ramiro.info')
WS.verifyElementPropertyValue(response, '[2].company.name', 'Romaguera-Jacobson')
WS.verifyElementPropertyValue(response, '[2].company.catchPhrase', 'Face to face bifurcated interface')
WS.verifyElementPropertyValue(response, '[2].company.bs', 'e-enable strategic applications')


WS.verifyElementPropertyValue(response, '[3].id', '4')
WS.verifyElementPropertyValue(response, '[3].name', 'Patricia Lebsack')
WS.verifyElementPropertyValue(response, '[3].username', 'Karianne')
WS.verifyElementPropertyValue(response, '[3].email', 'Julianne.OConner@kory.org')
WS.verifyElementPropertyValue(response, '[3].address.street', 'Hoeger Mall')
WS.verifyElementPropertyValue(response, '[3].address.suite', 'Apt. 692')
WS.verifyElementPropertyValue(response, '[3].address.city', 'South Elvis')
WS.verifyElementPropertyValue(response, '[3].address.zipcode', '53919-4257')
WS.verifyElementPropertyValue(response, '[3].address.geo.lat', '29.4572')
WS.verifyElementPropertyValue(response, '[3].address.geo.lng', '-164.2990')
WS.verifyElementPropertyValue(response, '[3].phone', '493-170-9623 x156')
WS.verifyElementPropertyValue(response, '[3].website', 'kale.biz')
WS.verifyElementPropertyValue(response, '[3].company.name', 'Robel-Corkery')
WS.verifyElementPropertyValue(response, '[3].company.catchPhrase', 'Multi-tiered zero tolerance productivity')
WS.verifyElementPropertyValue(response, '[3].company.bs', 'transition cutting-edge web services')


WS.verifyElementPropertyValue(response, '[4].id', '5')
WS.verifyElementPropertyValue(response, '[4].name', 'Chelsey Dietrich')
WS.verifyElementPropertyValue(response, '[4].username', 'Kamren')
WS.verifyElementPropertyValue(response, '[4].email', 'Lucio_Hettinger@annie.ca')
WS.verifyElementPropertyValue(response, '[4].address.street', 'Skiles Walks')
WS.verifyElementPropertyValue(response, '[4].address.suite', 'Suite 351')
WS.verifyElementPropertyValue(response, '[4].address.city', 'Roscoeview')
WS.verifyElementPropertyValue(response, '[4].address.zipcode', '33263')
WS.verifyElementPropertyValue(response, '[4].address.geo.lat', '-31.8129')
WS.verifyElementPropertyValue(response, '[4].address.geo.lng', '62.5342')
WS.verifyElementPropertyValue(response, '[4].phone', '(254)954-1289')
WS.verifyElementPropertyValue(response, '[4].website', 'demarco.info')
WS.verifyElementPropertyValue(response, '[4].company.name', 'Keebler LLC')
WS.verifyElementPropertyValue(response, '[4].company.catchPhrase', 'User-centric fault-tolerant solution')
WS.verifyElementPropertyValue(response, '[4].company.bs', 'revolutionize end-to-end systems')


WS.verifyElementPropertyValue(response, '[5].id', '6')
WS.verifyElementPropertyValue(response, '[5].name', 'Mrs. Dennis Schulist')
WS.verifyElementPropertyValue(response, '[5].username', 'Leopoldo_Corkery')
WS.verifyElementPropertyValue(response, '[5].email', 'Karley_Dach@jasper.info')
WS.verifyElementPropertyValue(response, '[5].address.street', 'Norberto Crossing')
WS.verifyElementPropertyValue(response, '[5].address.suite', 'Apt. 950')
WS.verifyElementPropertyValue(response, '[5].address.city', 'South Christy')
WS.verifyElementPropertyValue(response, '[5].address.zipcode', '23505-1337')
WS.verifyElementPropertyValue(response, '[5].address.geo.lat', '-71.4197')
WS.verifyElementPropertyValue(response, '[5].address.geo.lng', '71.7478')
WS.verifyElementPropertyValue(response, '[5].phone', '1-477-935-8478 x6430')
WS.verifyElementPropertyValue(response, '[5].website', 'ola.org')
WS.verifyElementPropertyValue(response, '[5].company.name', 'Considine-Lockman')
WS.verifyElementPropertyValue(response, '[5].company.catchPhrase', 'Synchronised bottom-line interface')
WS.verifyElementPropertyValue(response, '[5].company.bs', 'e-enable innovative applications')


WS.verifyElementPropertyValue(response, '[6].id', '7')
WS.verifyElementPropertyValue(response, '[6].name', 'Kurtis Weissnat')
WS.verifyElementPropertyValue(response, '[6].username', 'Elwyn.Skiles')
WS.verifyElementPropertyValue(response, '[6].email', 'Telly.Hoeger@billy.biz')
WS.verifyElementPropertyValue(response, '[6].address.street', 'Rex Trail')
WS.verifyElementPropertyValue(response, '[6].address.suite', 'Suite 280')
WS.verifyElementPropertyValue(response, '[6].address.city', 'Howemouth')
WS.verifyElementPropertyValue(response, '[6].address.zipcode', '58804-1099')
WS.verifyElementPropertyValue(response, '[6].address.geo.lat', '24.8918')
WS.verifyElementPropertyValue(response, '[6].address.geo.lng', '21.8984')
WS.verifyElementPropertyValue(response, '[6].phone', '210.067.6132')
WS.verifyElementPropertyValue(response, '[6].website', 'elvis.io')
WS.verifyElementPropertyValue(response, '[6].company.name', 'Johns Group')
WS.verifyElementPropertyValue(response, '[6].company.catchPhrase', 'Configurable multimedia task-force')
WS.verifyElementPropertyValue(response, '[6].company.bs', 'generate enterprise e-tailers')


WS.verifyElementPropertyValue(response, '[7].id', '8')
WS.verifyElementPropertyValue(response, '[7].name', 'Nicholas Runolfsdottir V')
WS.verifyElementPropertyValue(response, '[7].username', 'Maxime_Nienow')
WS.verifyElementPropertyValue(response, '[7].email', 'Sherwood@rosamond.me')
WS.verifyElementPropertyValue(response, '[7].address.street', 'Ellsworth Summit')
WS.verifyElementPropertyValue(response, '[7].address.suite', 'Suite 729')
WS.verifyElementPropertyValue(response, '[7].address.city', 'Aliyaview')
WS.verifyElementPropertyValue(response, '[7].address.zipcode', '45169')
WS.verifyElementPropertyValue(response, '[7].address.geo.lat', '-14.3990')
WS.verifyElementPropertyValue(response, '[7].address.geo.lng', '-120.7677')
WS.verifyElementPropertyValue(response, '[7].phone', '586.493.6943 x140')
WS.verifyElementPropertyValue(response, '[7].website', 'jacynthe.com')
WS.verifyElementPropertyValue(response, '[7].company.name', 'Abernathy Group')
WS.verifyElementPropertyValue(response, '[7].company.catchPhrase', 'Implemented secondary concept')
WS.verifyElementPropertyValue(response, '[7].company.bs', 'e-enable extensible e-tailers')


WS.verifyElementPropertyValue(response, '[8].id', '9')
WS.verifyElementPropertyValue(response, '[8].name', 'Glenna Reichert')
WS.verifyElementPropertyValue(response, '[8].username', 'Delphine')
WS.verifyElementPropertyValue(response, '[8].email', 'Chaim_McDermott@dana.io')
WS.verifyElementPropertyValue(response, '[8].address.street', 'Dayna Park')
WS.verifyElementPropertyValue(response, '[8].address.suite', 'Suite 449')
WS.verifyElementPropertyValue(response, '[8].address.city', 'Bartholomebury')
WS.verifyElementPropertyValue(response, '[8].address.zipcode', '76495-3109')
WS.verifyElementPropertyValue(response, '[8].address.geo.lat', '24.6463')
WS.verifyElementPropertyValue(response, '[8].address.geo.lng', '-168.8889')
WS.verifyElementPropertyValue(response, '[8].phone', '(775)976-6794 x41206')
WS.verifyElementPropertyValue(response, '[8].website', 'conrad.com')
WS.verifyElementPropertyValue(response, '[8].company.name', 'Yost and Sons')
WS.verifyElementPropertyValue(response, '[8].company.catchPhrase', 'Switchable contextually-based project')
WS.verifyElementPropertyValue(response, '[8].company.bs', 'aggregate real-time technologies')


WS.verifyElementPropertyValue(response, '[9].id', '10')
WS.verifyElementPropertyValue(response, '[9].name', 'Clementina DuBuque')
WS.verifyElementPropertyValue(response, '[9].username', 'Moriah.Stanton')
WS.verifyElementPropertyValue(response, '[9].email', 'Rey.Padberg@karina.biz')
WS.verifyElementPropertyValue(response, '[9].address.street', 'Kattie Turnpike')
WS.verifyElementPropertyValue(response, '[9].address.suite', 'Suite 198')
WS.verifyElementPropertyValue(response, '[9].address.city', 'Lebsackbury')
WS.verifyElementPropertyValue(response, '[9].address.zipcode', '31428-2261')
WS.verifyElementPropertyValue(response, '[9].address.geo.lat', '-38.2386')
WS.verifyElementPropertyValue(response, '[9].address.geo.lng', '57.2232')
WS.verifyElementPropertyValue(response, '[9].phone', '024-648-3804')
WS.verifyElementPropertyValue(response, '[9].website', 'ambrose.net')
WS.verifyElementPropertyValue(response, '[9].company.name', 'Hoeger LLC')
WS.verifyElementPropertyValue(response, '[9].company.catchPhrase', 'Centralized empowering task-force')
WS.verifyElementPropertyValue(response, '[9].company.bs', 'target end-to-end models')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
