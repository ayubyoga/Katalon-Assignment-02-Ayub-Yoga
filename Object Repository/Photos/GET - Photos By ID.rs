<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET - Photos By ID</name>
   <tag></tag>
   <elementGuidId>bca2e52e-03e1-4307-bd24-6b6a38e842c0</elementGuidId>
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
   <restUrl>https://jsonplaceholder.typicode.com/photos/6</restUrl>
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


WS.verifyElementPropertyValue(response, '[0].albumId', '1')
WS.verifyElementPropertyValue(response, '[0].id', '1')
WS.verifyElementPropertyValue(response, '[0].title', 'accusamus beatae ad facilis cum similique qui sunt')
WS.verifyElementPropertyValue(response, '[0].url', 'https://via.placeholder.com/600/92c952')
WS.verifyElementPropertyValue(response, '[0].thumbnailUrl', 'https://via.placeholder.com/150/92c952')


WS.verifyElementPropertyValue(response, '[1].albumId', '1')
WS.verifyElementPropertyValue(response, '[1].id', '2')
WS.verifyElementPropertyValue(response, '[1].title', 'reprehenderit est deserunt velit ipsam')
WS.verifyElementPropertyValue(response, '[1].url', 'https://via.placeholder.com/600/771796')
WS.verifyElementPropertyValue(response, '[1].thumbnailUrl', 'https://via.placeholder.com/150/771796')


WS.verifyElementPropertyValue(response, '[2].albumId', '1')
WS.verifyElementPropertyValue(response, '[2].id', '3')
WS.verifyElementPropertyValue(response, '[2].title', 'officia porro iure quia iusto qui ipsa ut modi')
WS.verifyElementPropertyValue(response, '[2].url', 'https://via.placeholder.com/600/24f355')
WS.verifyElementPropertyValue(response, '[2].thumbnailUrl', 'https://via.placeholder.com/150/24f355')


WS.verifyElementPropertyValue(response, '[3].albumId', '1')
WS.verifyElementPropertyValue(response, '[3].id', '4')
WS.verifyElementPropertyValue(response, '[3].title', 'culpa odio esse rerum omnis laboriosam voluptate repudiandae')
WS.verifyElementPropertyValue(response, '[3].url', 'https://via.placeholder.com/600/d32776')
WS.verifyElementPropertyValue(response, '[3].thumbnailUrl', 'https://via.placeholder.com/150/d32776')


WS.verifyElementPropertyValue(response, '[4].albumId', '1')
WS.verifyElementPropertyValue(response, '[4].id', '5')
WS.verifyElementPropertyValue(response, '[4].title', 'natus nisi omnis corporis facere molestiae rerum in')
WS.verifyElementPropertyValue(response, '[4].url', 'https://via.placeholder.com/600/f66b97')
WS.verifyElementPropertyValue(response, '[4].thumbnailUrl', 'https://via.placeholder.com/150/f66b97')


WS.verifyElementPropertyValue(response, '[5].albumId', '1')
WS.verifyElementPropertyValue(response, '[5].id', '6')
WS.verifyElementPropertyValue(response, '[5].title', 'accusamus ea aliquid et amet sequi nemo')
WS.verifyElementPropertyValue(response, '[5].url', 'https://via.placeholder.com/600/56a8c2')
WS.verifyElementPropertyValue(response, '[5].thumbnailUrl', 'https://via.placeholder.com/150/56a8c2')


WS.verifyElementPropertyValue(response, '[6].albumId', '1')
WS.verifyElementPropertyValue(response, '[6].id', '7')
WS.verifyElementPropertyValue(response, '[6].title', 'officia delectus consequatur vero aut veniam explicabo molestias')
WS.verifyElementPropertyValue(response, '[6].url', 'https://via.placeholder.com/600/b0f7cc')
WS.verifyElementPropertyValue(response, '[6].thumbnailUrl', 'https://via.placeholder.com/150/b0f7cc')


WS.verifyElementPropertyValue(response, '[7].albumId', '1')
WS.verifyElementPropertyValue(response, '[7].id', '8')
WS.verifyElementPropertyValue(response, '[7].title', 'aut porro officiis laborum odit ea laudantium corporis')
WS.verifyElementPropertyValue(response, '[7].url', 'https://via.placeholder.com/600/54176f')
WS.verifyElementPropertyValue(response, '[7].thumbnailUrl', 'https://via.placeholder.com/150/54176f')


WS.verifyElementPropertyValue(response, '[8].albumId', '1')
WS.verifyElementPropertyValue(response, '[8].id', '9')
WS.verifyElementPropertyValue(response, '[8].title', 'qui eius qui autem sed')
WS.verifyElementPropertyValue(response, '[8].url', 'https://via.placeholder.com/600/51aa97')
WS.verifyElementPropertyValue(response, '[8].thumbnailUrl', 'https://via.placeholder.com/150/51aa97')


WS.verifyElementPropertyValue(response, '[9].albumId', '1')
WS.verifyElementPropertyValue(response, '[9].id', '10')
WS.verifyElementPropertyValue(response, '[9].title', 'beatae et provident et ut vel')
WS.verifyElementPropertyValue(response, '[9].url', 'https://via.placeholder.com/600/810b14')
WS.verifyElementPropertyValue(response, '[9].thumbnailUrl', 'https://via.placeholder.com/150/810b14')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
